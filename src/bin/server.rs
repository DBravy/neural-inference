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
    #[serde(default)]
    timezone_offset_minutes: Option<i64>,
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
    let tz_offset = req.timezone_offset_minutes.unwrap_or(0);
    let shifted_events = if tz_offset == 0 {
        event_data.events.clone()
    } else {
        shift_events_by_offset(&event_data.events, tz_offset)
    };
    
    // Create estimator
    let estimator = PrimitiveEstimator::new();
    
    // Generate timeline
    let mut timeline = Vec::new();
    let start_time = Utc::now() - Duration::days(display_days);
    let end_time = Utc::now();
    
    let mut current_time = start_time;
    while current_time <= end_time {
        let result = estimator.estimate_at_time(&shifted_events, current_time);
        
        // Extract primitive scores
        let mut primitives = HashMap::new();
        for (key, state) in &result.primitives {
            primitives.insert(key.clone(), state.modified_score);
        }
        
        timeline.push(TimelinePoint {
            timestamp: current_time,
            primitives,
        });
        
        current_time = current_time + Duration::hours(req.resolution_hours);
    }
    
    // Get final state
    let final_result = estimator.estimate_at_time(&shifted_events, end_time);
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    println!("🧠 Neural Primitive Estimator Server");
    println!("=====================================");
    println!("Starting server at http://localhost:8080");
    println!("Open your browser and navigate to http://localhost:8080");
    println!();
    
    HttpServer::new(|| {
        App::new()
            .route("/api/profiles", web::get().to(get_profiles))
            .route("/api/estimate", web::post().to(estimate_profile))
            .route("/api/chat", web::post().to(chat_endpoint))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

