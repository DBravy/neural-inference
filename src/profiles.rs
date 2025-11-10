// profiles.rs - Predefined data profiles for demo
use crate::{Event, EventData};
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub schedule: Vec<Event>,
}

// Helper functions for adding randomness
fn add_time_jitter(time: DateTime<Utc>, rng: &mut StdRng, minutes_range: i64) -> DateTime<Utc> {
    let jitter_minutes = rng.gen_range(-minutes_range..=minutes_range);
    time + Duration::minutes(jitter_minutes)
}

fn add_value_jitter(value: f64, rng: &mut StdRng, percentage: f64) -> f64 {
    let jitter = rng.gen_range(-percentage..=percentage);
    value * (1.0 + jitter / 100.0)
}

fn create_rng(profile_id: &str, day: i64) -> StdRng {
    // Create a deterministic but varied seed based on profile and day
    let seed = format!("{}{}", profile_id, day);
    let seed_bytes: [u8; 32] = {
        let mut bytes = [0u8; 32];
        let seed_hash = seed.as_bytes();
        for (i, &byte) in seed_hash.iter().take(32).enumerate() {
            bytes[i] = byte;
        }
        bytes
    };
    StdRng::from_seed(seed_bytes)
}

pub fn get_all_profiles() -> Vec<Profile> {
    // Generate schedule with same window as estimation system for consistency
    // The estimation system uses 4 display days + 7 padding days = 11 total
    // We must use the same window to ensure event timestamps match
    let display_days = 4;   // Matches estimate_profile endpoint
    let padding_days = 7;   // Matches max ContextConfig window (168h)
    let total_days = display_days + padding_days;
    
    // Round to start of current day to ensure consistent base_time across all calls
    let now = Utc::now();
    let today_start = now.with_hour(0).unwrap().with_minute(0).unwrap()
        .with_second(0).unwrap().with_nanosecond(0).unwrap();
    let base_time = today_start - Duration::days(total_days);
    
    vec![
        Profile {
            id: "healthy".to_string(),
            name: "Healthy Routine".to_string(),
            description: "Consistent sleep, balanced meals, regular exercise, and good habits".to_string(),
            schedule: generate_healthy_routine(base_time, total_days),
        },
        Profile {
            id: "sleep_deprived".to_string(),
            name: "Finals Week".to_string(),
            description: "Multiple nights of poor sleep with high caffeine use".to_string(),
            schedule: generate_sleep_deprived(base_time, total_days),
        },
        Profile {
            id: "high_stress".to_string(),
            name: "High Stress".to_string(),
            description: "Work pressure, frequent stress events, and irregular eating".to_string(),
            schedule: generate_high_stress(base_time, total_days),
        },
        Profile {
            id: "athlete".to_string(),
            name: "Athlete Training".to_string(),
            description: "Intense exercise routine with optimized nutrition and recovery".to_string(),
            schedule: generate_athlete(base_time, total_days),
        },
        Profile {
            id: "shift_worker".to_string(),
            name: "Shift Worker".to_string(),
            description: "Irregular sleep schedule with circadian misalignment".to_string(),
            schedule: generate_shift_worker(base_time, total_days),
        },
    ]
}

pub fn generate_profile_events(profile_id: &str, days: i64) -> EventData {
    // Round to start of current day to ensure consistent base_time across all calls
    // This ensures events match those shown in the schedule UI
    let now = Utc::now();
    let today_start = now.with_hour(0).unwrap().with_minute(0).unwrap()
        .with_second(0).unwrap().with_nanosecond(0).unwrap();
    let base_time = today_start - Duration::days(days);
    
    let events = match profile_id {
        "healthy" => generate_healthy_routine(base_time, days),
        "sleep_deprived" => generate_sleep_deprived(base_time, days),
        "high_stress" => generate_high_stress(base_time, days),
        "athlete" => generate_athlete(base_time, days),
        "shift_worker" => generate_shift_worker(base_time, days),
        _ => generate_healthy_routine(base_time, days),
    };
    
    EventData {
        user_id: "demo_user".to_string(),
        events,
    }
}

fn generate_healthy_routine(base_time: DateTime<Utc>, days: i64) -> Vec<Event> {
    let mut events = Vec::new();
    let mut event_counter = 1;
    
    for day in 0..=days {
        let day_start = base_time + Duration::days(day);
        let mut rng = create_rng("healthy", day);
        
        // Sleep: 11 PM - 7 AM (8 hours, good quality) with jitter
        let sleep_start_base = day_start.with_hour(23).unwrap().with_minute(0).unwrap();
        let sleep_start = add_time_jitter(sleep_start_base, &mut rng, 15);
        let sleep_duration = add_value_jitter(8.0, &mut rng, 5.0);
        let sleep_efficiency = add_value_jitter(0.88, &mut rng, 3.0).min(0.98).max(0.75);
        
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes((sleep_duration * 60.0) as i64)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(sleep_duration)),
                ("quality".to_string(), json!("good")),
                ("sleep_efficiency".to_string(), json!(sleep_efficiency)),
            ]),
        });
        event_counter += 1;
        
        // Wake up
        let wake_time = sleep_start + Duration::minutes((sleep_duration * 60.0) as i64);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::from([
                ("natural_wake".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Morning HRV (good) with jitter
        let hrv_value = add_value_jitter(68.0, &mut rng, 8.0);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(15), &mut rng, 5),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(hrv_value)),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Morning light exposure with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "light_exposure".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(30), &mut rng, 10),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity_lux".to_string(), json!(add_value_jitter(8000.0, &mut rng, 15.0))),
                ("duration_minutes".to_string(), json!(add_value_jitter(20.0, &mut rng, 20.0))),
            ]),
        });
        event_counter += 1;
        
        // Breakfast (balanced) with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(1), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("breakfast")),
                ("carb_grams".to_string(), json!(add_value_jitter(45.0, &mut rng, 12.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(25.0, &mut rng, 12.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(15.0, &mut rng, 12.0))),
                ("glycemic_index".to_string(), json!("medium")),
                ("protein_percentage".to_string(), json!(add_value_jitter(22.0, &mut rng, 10.0))),
            ]),
        });
        event_counter += 1;
        
        // Morning caffeine (moderate) with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(1) + Duration::minutes(15), &mut rng, 10),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(add_value_jitter(100.0, &mut rng, 15.0))),
                ("form".to_string(), json!("coffee")),
            ]),
        });
        event_counter += 1;
        
        // Morning exercise with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "exercise".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(2), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(add_value_jitter(40.0, &mut rng, 15.0))),
                ("intensity".to_string(), json!("moderate")),
                ("type".to_string(), json!("cardio")),
                ("vo2max_percentage".to_string(), json!(add_value_jitter(65.0, &mut rng, 8.0))),
            ]),
        });
        event_counter += 1;
        
        // Lunch with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(5), &mut rng, 20),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("lunch")),
                ("carb_grams".to_string(), json!(add_value_jitter(60.0, &mut rng, 12.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(30.0, &mut rng, 12.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(20.0, &mut rng, 12.0))),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(add_value_jitter(27.0, &mut rng, 10.0))),
            ]),
        });
        event_counter += 1;
        
        // Positive social interaction with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "social_interaction".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(10), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(add_value_jitter(60.0, &mut rng, 25.0))),
                ("quality".to_string(), json!("positive")),
            ]),
        });
        event_counter += 1;
        
        // Dinner with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(11), &mut rng, 20),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("dinner")),
                ("carb_grams".to_string(), json!(add_value_jitter(50.0, &mut rng, 12.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(35.0, &mut rng, 12.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(18.0, &mut rng, 12.0))),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(add_value_jitter(34.0, &mut rng, 10.0))),
            ]),
        });
        event_counter += 1;
    }
    
    events
}

fn generate_sleep_deprived(base_time: DateTime<Utc>, days: i64) -> Vec<Event> {
    let mut events = Vec::new();
    let mut event_counter = 1;
    
    for day in 0..=days {
        let day_start = base_time + Duration::days(day);
        let mut rng = create_rng("sleep_deprived", day);
        
        // Poor sleep: 1 AM - 5:30 AM (4.5 hours, poor quality) with jitter
        let sleep_start_base = day_start.with_hour(1).unwrap().with_minute(0).unwrap();
        let sleep_start = add_time_jitter(sleep_start_base, &mut rng, 20);
        let sleep_duration = add_value_jitter(4.5, &mut rng, 10.0);
        let sleep_efficiency = add_value_jitter(0.60, &mut rng, 8.0).min(0.70).max(0.45);
        
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes((sleep_duration * 60.0) as i64)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(sleep_duration)),
                ("quality".to_string(), json!("poor")),
                ("sleep_efficiency".to_string(), json!(sleep_efficiency)),
            ]),
        });
        event_counter += 1;
        
        // Forced wake
        let wake_time = sleep_start + Duration::minutes((sleep_duration * 60.0) as i64);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::from([
                ("natural_wake".to_string(), json!(false)),
                ("alarm".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Low HRV with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(15), &mut rng, 5),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(28.0, &mut rng, 10.0))),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Elevated HR with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_heart_rate".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(15), &mut rng, 5),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(82.0, &mut rng, 7.0))),
                ("unit".to_string(), json!("bpm")),
            ]),
        });
        event_counter += 1;
        
        // High caffeine (desperate for energy) with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(30), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(add_value_jitter(200.0, &mut rng, 15.0))),
                ("form".to_string(), json!("coffee")),
            ]),
        });
        event_counter += 1;
        
        // Skipped breakfast (no time)
        
        // Another caffeine dose with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(3), &mut rng, 20),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(add_value_jitter(150.0, &mut rng, 20.0))),
                ("form".to_string(), json!("energy_drink")),
            ]),
        });
        event_counter += 1;
        
        // Quick high-GI lunch with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(6), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("lunch")),
                ("carb_grams".to_string(), json!(add_value_jitter(80.0, &mut rng, 15.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(10.0, &mut rng, 20.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(15.0, &mut rng, 15.0))),
                ("glycemic_index".to_string(), json!("high")),
                ("protein_percentage".to_string(), json!(add_value_jitter(9.0, &mut rng, 15.0))),
            ]),
        });
        event_counter += 1;
        
        // Stress event with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "stress_event".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(8), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity".to_string(), json!("high")),
                ("controllable".to_string(), json!(false)),
                ("social_evaluative".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Evening screen time (late) with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "screen_time".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(17), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(add_value_jitter(120.0, &mut rng, 20.0))),
                ("blue_light_intensity".to_string(), json!("high")),
                ("hours_before_sleep".to_string(), json!(add_value_jitter(2.0, &mut rng, 25.0))),
            ]),
        });
        event_counter += 1;
    }
    
    events
}

fn generate_high_stress(base_time: DateTime<Utc>, days: i64) -> Vec<Event> {
    let mut events = Vec::new();
    let mut event_counter = 1;
    
    for day in 0..=days {
        let day_start = base_time + Duration::days(day);
        let mut rng = create_rng("high_stress", day);
        
        // Moderate sleep: 12 AM - 6 AM (6 hours, fair quality) with jitter
        let sleep_start_base = day_start.with_hour(0).unwrap().with_minute(0).unwrap();
        let sleep_start = add_time_jitter(sleep_start_base, &mut rng, 20);
        let sleep_duration = add_value_jitter(6.0, &mut rng, 10.0);
        let sleep_efficiency = add_value_jitter(0.72, &mut rng, 8.0).min(0.85).max(0.55);
        
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes((sleep_duration * 60.0) as i64)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(sleep_duration)),
                ("quality".to_string(), json!("fair")),
                ("sleep_efficiency".to_string(), json!(sleep_efficiency)),
            ]),
        });
        event_counter += 1;
        
        let wake_time = sleep_start + Duration::minutes((sleep_duration * 60.0) as i64);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::new(),
        });
        event_counter += 1;
        
        // Reduced HRV with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(15), &mut rng, 5),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(38.0, &mut rng, 12.0))),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Caffeine with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(30), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(add_value_jitter(150.0, &mut rng, 18.0))),
            ]),
        });
        event_counter += 1;
        
        // Morning stress with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "stress_event".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(2), &mut rng, 25),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity".to_string(), json!("moderate")),
                ("controllable".to_string(), json!(false)),
                ("social_evaluative".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Interruptions with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "interruption".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(4), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("frequency".to_string(), json!(add_value_jitter(8.0, &mut rng, 20.0))),
                ("total_disruption_minutes".to_string(), json!(add_value_jitter(35.0, &mut rng, 25.0))),
            ]),
        });
        event_counter += 1;
        
        // Skipped lunch (too busy)
        
        // Afternoon stress with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "stress_event".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(8), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity".to_string(), json!("high")),
                ("controllable".to_string(), json!(false)),
                ("social_evaluative".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Elevated HR (sustained) with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_heart_rate".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(9), &mut rng, 20),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(88.0, &mut rng, 8.0))),
                ("unit".to_string(), json!("bpm")),
            ]),
        });
        event_counter += 1;
        
        // Elevated respiratory rate with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_respiratory_rate".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(9), &mut rng, 20),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(21.0, &mut rng, 8.0))),
                ("unit".to_string(), json!("breaths/min")),
            ]),
        });
        event_counter += 1;
        
        // Late dinner with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(14), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("dinner")),
                ("carb_grams".to_string(), json!(add_value_jitter(70.0, &mut rng, 15.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(20.0, &mut rng, 15.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(25.0, &mut rng, 15.0))),
                ("glycemic_index".to_string(), json!("high")),
            ]),
        });
        event_counter += 1;
    }
    
    events
}

fn generate_athlete(base_time: DateTime<Utc>, days: i64) -> Vec<Event> {
    let mut events = Vec::new();
    let mut event_counter = 1;
    
    for day in 0..=days {
        let day_start = base_time + Duration::days(day);
        let mut rng = create_rng("athlete", day);
        
        // Good sleep: 10 PM - 6:30 AM (8.5 hours, excellent quality) with jitter
        let sleep_start_base = day_start.with_hour(22).unwrap().with_minute(0).unwrap();
        let sleep_start = add_time_jitter(sleep_start_base, &mut rng, 15);
        let sleep_duration = add_value_jitter(8.5, &mut rng, 5.0);
        let sleep_efficiency = add_value_jitter(0.92, &mut rng, 3.0).min(0.98).max(0.85);
        
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes((sleep_duration * 60.0) as i64)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(sleep_duration)),
                ("quality".to_string(), json!("excellent")),
                ("sleep_efficiency".to_string(), json!(sleep_efficiency)),
            ]),
        });
        event_counter += 1;
        
        let wake_time = sleep_start + Duration::minutes((sleep_duration * 60.0) as i64);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::new(),
        });
        event_counter += 1;
        
        // High HRV with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(15), &mut rng, 5),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(85.0, &mut rng, 8.0))),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Morning light with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "light_exposure".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(30), &mut rng, 10),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity_lux".to_string(), json!(add_value_jitter(10000.0, &mut rng, 15.0))),
                ("duration_minutes".to_string(), json!(add_value_jitter(30.0, &mut rng, 15.0))),
            ]),
        });
        event_counter += 1;
        
        // High-protein breakfast with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(1), &mut rng, 10),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("breakfast")),
                ("carb_grams".to_string(), json!(add_value_jitter(50.0, &mut rng, 10.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(40.0, &mut rng, 10.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(20.0, &mut rng, 10.0))),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(add_value_jitter(36.0, &mut rng, 8.0))),
            ]),
        });
        event_counter += 1;
        
        // Morning HIIT with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "exercise".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(2), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(add_value_jitter(45.0, &mut rng, 10.0))),
                ("intensity".to_string(), json!("high_intensity")),
                ("type".to_string(), json!("hiit")),
                ("vo2max_percentage".to_string(), json!(add_value_jitter(85.0, &mut rng, 6.0))),
            ]),
        });
        event_counter += 1;
        
        // Post-workout meal with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(3), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("post_workout")),
                ("carb_grams".to_string(), json!(add_value_jitter(60.0, &mut rng, 12.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(35.0, &mut rng, 12.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(10.0, &mut rng, 15.0))),
                ("glycemic_index".to_string(), json!("medium")),
            ]),
        });
        event_counter += 1;
        
        // Afternoon steady cardio with jitter
        if day % 2 == 0 {
            events.push(Event {
                event_id: format!("evt_{}", event_counter),
                event_type: "exercise".to_string(),
                timestamp: add_time_jitter(wake_time + Duration::hours(8), &mut rng, 20),
                end_timestamp: None,
                properties: HashMap::from([
                    ("duration_minutes".to_string(), json!(add_value_jitter(60.0, &mut rng, 12.0))),
                    ("intensity".to_string(), json!("moderate")),
                    ("type".to_string(), json!("cardio")),
                    ("vo2max_percentage".to_string(), json!(add_value_jitter(65.0, &mut rng, 8.0))),
                ]),
            });
            event_counter += 1;
        }
        
        // High-protein dinner with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(11), &mut rng, 20),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("dinner")),
                ("carb_grams".to_string(), json!(add_value_jitter(55.0, &mut rng, 10.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(45.0, &mut rng, 10.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(20.0, &mut rng, 10.0))),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(add_value_jitter(38.0, &mut rng, 8.0))),
            ]),
        });
        event_counter += 1;
        
        // Good glucose control with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_blood_glucose".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(12), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(92.0, &mut rng, 6.0))),
                ("unit".to_string(), json!("mg/dL")),
            ]),
        });
        event_counter += 1;
    }
    
    events
}

fn generate_shift_worker(base_time: DateTime<Utc>, days: i64) -> Vec<Event> {
    let mut events = Vec::new();
    let mut event_counter = 1;
    
    for day in 0..=days {
        let day_start = base_time + Duration::days(day);
        let mut rng = create_rng("shift_worker", day);
        
        // Rotating sleep schedule with jitter
        let sleep_hour = if day % 3 == 0 {
            23 // Normal night
        } else if day % 3 == 1 {
            8 // Morning sleep (night shift)
        } else {
            15 // Afternoon sleep (evening shift)
        };
        
        let sleep_start_base = day_start.with_hour(sleep_hour).unwrap().with_minute(0).unwrap();
        let sleep_start = add_time_jitter(sleep_start_base, &mut rng, 25);
        let sleep_duration = add_value_jitter(6.0, &mut rng, 12.0);
        let base_efficiency = if day % 3 == 0 { 0.75 } else { 0.62 };
        let sleep_efficiency = add_value_jitter(base_efficiency, &mut rng, 10.0).min(0.85).max(0.45);
        
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes((sleep_duration * 60.0) as i64)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(sleep_duration)),
                ("quality".to_string(), json!(if day % 3 == 0 { "fair" } else { "poor" })),
                ("sleep_efficiency".to_string(), json!(sleep_efficiency)),
            ]),
        });
        event_counter += 1;
        
        let wake_time = sleep_start + Duration::minutes((sleep_duration * 60.0) as i64);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::new(),
        });
        event_counter += 1;
        
        // Variable HRV with jitter
        let base_hrv = if day % 3 == 0 { 55.0 } else { 38.0 };
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(15), &mut rng, 5),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(add_value_jitter(base_hrv, &mut rng, 12.0))),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // High caffeine dependency with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::minutes(30), &mut rng, 15),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(add_value_jitter(200.0, &mut rng, 20.0))),
            ]),
        });
        event_counter += 1;
        
        // Irregular meals with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(2), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("carb_grams".to_string(), json!(add_value_jitter(60.0, &mut rng, 15.0))),
                ("protein_grams".to_string(), json!(add_value_jitter(20.0, &mut rng, 20.0))),
                ("fat_grams".to_string(), json!(add_value_jitter(18.0, &mut rng, 15.0))),
                ("glycemic_index".to_string(), json!("medium")),
            ]),
        });
        event_counter += 1;
        
        // Another caffeine dose with jitter
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: add_time_jitter(wake_time + Duration::hours(6), &mut rng, 30),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(add_value_jitter(150.0, &mut rng, 25.0))),
            ]),
        });
        event_counter += 1;
        
        // Circadian misalignment - inappropriate light exposure with jitter
        if day % 3 != 0 {
            events.push(Event {
                event_id: format!("evt_{}", event_counter),
                event_type: "light_exposure".to_string(),
                timestamp: add_time_jitter(wake_time + Duration::hours(4), &mut rng, 30),
                end_timestamp: None,
                properties: HashMap::from([
                    ("intensity_lux".to_string(), json!(add_value_jitter(3000.0, &mut rng, 20.0))),
                    ("duration_minutes".to_string(), json!(add_value_jitter(60.0, &mut rng, 20.0))),
                ]),
            });
            event_counter += 1;
        }
    }
    
    events
}