// server.rs - Web server for neural primitive demo
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Duration, Utc};
use neurological_primitives::{
    generate_profile_events, get_all_profiles, PrimitiveEstimator,
    chat::ChatRequest,
    Event,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct EstimateRequest {
    profile_id: String,
    resolution_hours: i64,
    // timezone_offset_minutes is sent by the frontend but not used for estimation
    // (events are already in local time, so we don't need to adjust)
    #[serde(default)]
    #[allow(dead_code)]
    timezone_offset_minutes: Option<i64>,
    #[serde(default)]
    adhd_mode: bool,
}

#[derive(Debug, Serialize)]
struct TimelinePoint {
    timestamp: DateTime<Utc>,
    primitives: HashMap<String, f64>,
}

#[derive(Debug, Serialize)]
struct EstimateResponse {
    timeline: Vec<TimelinePoint>,
    final_state: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ProfilesQuery {
    #[serde(default)]
    tz_offset: Option<i64>,
}

fn shift_events_by_offset(events: &[Event], offset_minutes: i64) -> Vec<Event> {
    if offset_minutes == 0 {
        return events.to_vec();
    }
    let shift = Duration::minutes(offset_minutes);
    events
        .iter()
        .map(|e| {
            let mut ne = e.clone();
            ne.timestamp = e.timestamp + shift;
            ne.end_timestamp = e.end_timestamp.map(|t| t + shift);
            ne
        })
        .collect()
}

async fn get_profiles(query: web::Query<ProfilesQuery>) -> impl Responder {
    let offset = query.tz_offset.unwrap_or(0);
    let mut profiles = get_all_profiles();
    if offset != 0 {
        for profile in &mut profiles {
            profile.schedule = shift_events_by_offset(&profile.schedule, offset);
        }
    }
    HttpResponse::Ok().json(profiles)
}

async fn estimate_profile(req: web::Json<EstimateRequest>) -> impl Responder {
    // Generate events for the profile
    // Add lookback padding so long context windows (e.g., circadian 7d) have history
    let display_days: i64 = 4;
    let padding_days_before: i64 = 7; // matches max ContextConfig window (168h)
    let event_data = generate_profile_events(&req.profile_id, display_days + padding_days_before);
    
    // NOTE: We do NOT shift events for estimation. Profile events are generated with
    // UTC timestamps where the hour component represents local time (e.g., 23:00 UTC = 11 PM local).
    // Circadian calculations depend on these hour values being correct for local time.
    // We only shift timestamps for display purposes (when showing the schedule to users).
    let events_for_estimation = event_data.events.clone();
    
    // Create estimator with ADHD mode if requested
    let estimator = PrimitiveEstimator::with_adhd_mode(req.adhd_mode);
    
    // Convert real UTC to "fake UTC" for proper event comparison
    // Events are stored with UTC timestamps where the hour component represents local time
    // So we subtract the timezone offset to get "fake UTC" time
    let tz_offset_minutes = req.timezone_offset_minutes.unwrap_or(0);
    let real_utc_now = Utc::now();
    let fake_utc_now = real_utc_now - Duration::minutes(tz_offset_minutes);
    
    // Generate timeline
    let mut timeline = Vec::new();
    let start_time = fake_utc_now - Duration::days(display_days);
    let end_time = fake_utc_now;
    
    let mut current_time = start_time;
    while current_time <= end_time {
        let result = estimator.estimate_at_time(&events_for_estimation, current_time);
        
        // Extract primitive scores
        let mut primitives = HashMap::new();
        for (key, state) in &result.primitives {
            // For dopamine and serotonin, use effective_score (after reciprocal inhibition) if available
            let score = if (key == "dopamine" || key == "serotonin") && state.effective_score.is_some() {
                state.effective_score.unwrap()
            } else {
                state.modified_score
            };
            primitives.insert(key.clone(), score);
        }
        
        timeline.push(TimelinePoint {
            timestamp: current_time,
            primitives,
        });
        
        current_time = current_time + Duration::hours(req.resolution_hours);
    }
    
    // Get final state
    let final_result = estimator.estimate_at_time(&events_for_estimation, end_time);
    let final_state = serde_json::to_value(&final_result).unwrap();
    
    HttpResponse::Ok().json(EstimateResponse {
        timeline,
        final_state,
    })
}

async fn chat_endpoint(req: web::Json<ChatRequest>) -> impl Responder {
    match neurological_primitives::chat::chat_reply(req.into_inner()).await {
        Ok(reply) => HttpResponse::Ok().json(reply),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": err })),
    }
}

async fn health_check() -> impl Responder {
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./static".to_string());
    let static_exists = std::path::Path::new(&static_dir).exists();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "server": "Neural Primitive Estimator",
        "cwd": cwd,
        "static_dir": static_dir,
        "static_exists": static_exists,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get port from environment variable (Railway) or default to 8080
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    
    let bind_address = format!("0.0.0.0:{}", port);
    
    // Determine static directory path
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./static".to_string());
    
    println!("🧠 Neural Primitive Estimator Server");
    println!("=====================================");
    println!("Starting server at {}", bind_address);
    println!("Static files directory: {}", static_dir);
    println!("Current working directory: {:?}", std::env::current_dir().unwrap());
    
    // Check if static directory exists
    if !std::path::Path::new(&static_dir).exists() {
        eprintln!("⚠️  WARNING: Static directory '{}' does not exist!", static_dir);
        eprintln!("    The server will start but static files will not be served.");
    }
    println!();
    
    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/api/profiles", web::get().to(get_profiles))
            .route("/api/estimate", web::post().to(estimate_profile))
            .route("/api/chat", web::post().to(chat_endpoint))
            .service(fs::Files::new("/", static_dir.clone()).index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await
}

