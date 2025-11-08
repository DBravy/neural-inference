// profiles.rs - Predefined data profiles for demo
use crate::{Event, EventData};
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub schedule: Vec<Event>,
}

pub fn get_all_profiles() -> Vec<Profile> {
    // Generate schedule for display purposes (3 days worth of events)
    let display_days = 3;
    let base_time = Utc::now() - Duration::days(display_days);
    
    vec![
        Profile {
            id: "healthy".to_string(),
            name: "Healthy Routine".to_string(),
            description: "Consistent sleep, balanced meals, regular exercise, and good habits".to_string(),
            schedule: generate_healthy_routine(base_time, display_days),
        },
        Profile {
            id: "sleep_deprived".to_string(),
            name: "Sleep Deprived".to_string(),
            description: "Multiple nights of poor sleep with high caffeine use".to_string(),
            schedule: generate_sleep_deprived(base_time, display_days),
        },
        Profile {
            id: "high_stress".to_string(),
            name: "High Stress".to_string(),
            description: "Work pressure, frequent stress events, and irregular eating".to_string(),
            schedule: generate_high_stress(base_time, display_days),
        },
        Profile {
            id: "athlete".to_string(),
            name: "Athlete Training".to_string(),
            description: "Intense exercise routine with optimized nutrition and recovery".to_string(),
            schedule: generate_athlete(base_time, display_days),
        },
        Profile {
            id: "shift_worker".to_string(),
            name: "Shift Worker".to_string(),
            description: "Irregular sleep schedule with circadian misalignment".to_string(),
            schedule: generate_shift_worker(base_time, display_days),
        },
    ]
}

pub fn generate_profile_events(profile_id: &str, days: i64) -> EventData {
    let base_time = Utc::now() - Duration::days(days);
    
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
        
        // Sleep: 11 PM - 7 AM (8 hours, good quality)
        let sleep_start = day_start.with_hour(23).unwrap().with_minute(0).unwrap();
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::hours(8)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(8.0)),
                ("quality".to_string(), json!("good")),
                ("sleep_efficiency".to_string(), json!(0.88)),
            ]),
        });
        event_counter += 1;
        
        // Wake up
        let wake_time = sleep_start + Duration::hours(8);
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
        
        // Morning HRV (good)
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: wake_time + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(68.0 + (day as f64 % 5.0))),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Morning light exposure
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "light_exposure".to_string(),
            timestamp: wake_time + Duration::minutes(30),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity_lux".to_string(), json!(8000.0)),
                ("duration_minutes".to_string(), json!(20.0)),
            ]),
        });
        event_counter += 1;
        
        // Breakfast (balanced)
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(1),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("breakfast")),
                ("carb_grams".to_string(), json!(45.0)),
                ("protein_grams".to_string(), json!(25.0)),
                ("fat_grams".to_string(), json!(15.0)),
                ("glycemic_index".to_string(), json!("medium")),
                ("protein_percentage".to_string(), json!(22.0)),
            ]),
        });
        event_counter += 1;
        
        // Morning caffeine (moderate)
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: wake_time + Duration::hours(1) + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(100.0)),
                ("form".to_string(), json!("coffee")),
            ]),
        });
        event_counter += 1;
        
        // Morning exercise
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "exercise".to_string(),
            timestamp: wake_time + Duration::hours(2),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(40.0)),
                ("intensity".to_string(), json!("moderate")),
                ("type".to_string(), json!("cardio")),
                ("vo2max_percentage".to_string(), json!(65.0)),
            ]),
        });
        event_counter += 1;
        
        // Lunch
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(5),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("lunch")),
                ("carb_grams".to_string(), json!(60.0)),
                ("protein_grams".to_string(), json!(30.0)),
                ("fat_grams".to_string(), json!(20.0)),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(27.0)),
            ]),
        });
        event_counter += 1;
        
        // Positive social interaction
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "social_interaction".to_string(),
            timestamp: wake_time + Duration::hours(10),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(60.0)),
                ("quality".to_string(), json!("positive")),
            ]),
        });
        event_counter += 1;
        
        // Dinner
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(11),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("dinner")),
                ("carb_grams".to_string(), json!(50.0)),
                ("protein_grams".to_string(), json!(35.0)),
                ("fat_grams".to_string(), json!(18.0)),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(34.0)),
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
        
        // Poor sleep: 1 AM - 5:30 AM (4.5 hours, poor quality)
        let sleep_start = day_start.with_hour(1).unwrap().with_minute(0).unwrap();
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes(270)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(4.5)),
                ("quality".to_string(), json!("poor")),
                ("sleep_efficiency".to_string(), json!(0.60)),
            ]),
        });
        event_counter += 1;
        
        // Forced wake
        let wake_time = sleep_start + Duration::minutes(270);
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
        
        // Low HRV
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: wake_time + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(28.0 + (day as f64 % 3.0))),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Elevated HR
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_heart_rate".to_string(),
            timestamp: wake_time + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(82.0)),
                ("unit".to_string(), json!("bpm")),
            ]),
        });
        event_counter += 1;
        
        // High caffeine (desperate for energy)
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: wake_time + Duration::minutes(30),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(200.0)),
                ("form".to_string(), json!("coffee")),
            ]),
        });
        event_counter += 1;
        
        // Skipped breakfast (no time)
        
        // Another caffeine dose
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: wake_time + Duration::hours(3),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(150.0)),
                ("form".to_string(), json!("energy_drink")),
            ]),
        });
        event_counter += 1;
        
        // Quick high-GI lunch
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(6),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("lunch")),
                ("carb_grams".to_string(), json!(80.0)),
                ("protein_grams".to_string(), json!(10.0)),
                ("fat_grams".to_string(), json!(15.0)),
                ("glycemic_index".to_string(), json!("high")),
                ("protein_percentage".to_string(), json!(9.0)),
            ]),
        });
        event_counter += 1;
        
        // Stress event
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "stress_event".to_string(),
            timestamp: wake_time + Duration::hours(8),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity".to_string(), json!("high")),
                ("controllable".to_string(), json!(false)),
                ("social_evaluative".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Evening screen time (late)
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "screen_time".to_string(),
            timestamp: wake_time + Duration::hours(17),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(120.0)),
                ("blue_light_intensity".to_string(), json!("high")),
                ("hours_before_sleep".to_string(), json!(2.0)),
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
        
        // Moderate sleep: 12 AM - 6 AM (6 hours, fair quality)
        let sleep_start = day_start.with_hour(0).unwrap().with_minute(0).unwrap();
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::hours(6)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(6.0)),
                ("quality".to_string(), json!("fair")),
                ("sleep_efficiency".to_string(), json!(0.72)),
            ]),
        });
        event_counter += 1;
        
        let wake_time = sleep_start + Duration::hours(6);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::new(),
        });
        event_counter += 1;
        
        // Reduced HRV
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: wake_time + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(38.0)),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Caffeine
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: wake_time + Duration::minutes(30),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(150.0)),
            ]),
        });
        event_counter += 1;
        
        // Morning stress
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "stress_event".to_string(),
            timestamp: wake_time + Duration::hours(2),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity".to_string(), json!("moderate")),
                ("controllable".to_string(), json!(false)),
                ("social_evaluative".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Interruptions
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "interruption".to_string(),
            timestamp: wake_time + Duration::hours(4),
            end_timestamp: None,
            properties: HashMap::from([
                ("frequency".to_string(), json!(8.0)),
                ("total_disruption_minutes".to_string(), json!(35.0)),
            ]),
        });
        event_counter += 1;
        
        // Skipped lunch (too busy)
        
        // Afternoon stress
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "stress_event".to_string(),
            timestamp: wake_time + Duration::hours(8),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity".to_string(), json!("high")),
                ("controllable".to_string(), json!(false)),
                ("social_evaluative".to_string(), json!(true)),
            ]),
        });
        event_counter += 1;
        
        // Elevated HR (sustained)
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_heart_rate".to_string(),
            timestamp: wake_time + Duration::hours(9),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(88.0)),
                ("unit".to_string(), json!("bpm")),
            ]),
        });
        event_counter += 1;
        
        // Elevated respiratory rate
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_respiratory_rate".to_string(),
            timestamp: wake_time + Duration::hours(9),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(21.0)),
                ("unit".to_string(), json!("breaths/min")),
            ]),
        });
        event_counter += 1;
        
        // Late dinner
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(14),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("dinner")),
                ("carb_grams".to_string(), json!(70.0)),
                ("protein_grams".to_string(), json!(20.0)),
                ("fat_grams".to_string(), json!(25.0)),
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
        
        // Good sleep: 10 PM - 6:30 AM (8.5 hours, excellent quality)
        let sleep_start = day_start.with_hour(22).unwrap().with_minute(0).unwrap();
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::minutes(510)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(8.5)),
                ("quality".to_string(), json!("excellent")),
                ("sleep_efficiency".to_string(), json!(0.92)),
            ]),
        });
        event_counter += 1;
        
        let wake_time = sleep_start + Duration::minutes(510);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::new(),
        });
        event_counter += 1;
        
        // High HRV
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: wake_time + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(85.0)),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // Morning light
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "light_exposure".to_string(),
            timestamp: wake_time + Duration::minutes(30),
            end_timestamp: None,
            properties: HashMap::from([
                ("intensity_lux".to_string(), json!(10000.0)),
                ("duration_minutes".to_string(), json!(30.0)),
            ]),
        });
        event_counter += 1;
        
        // High-protein breakfast
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(1),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("breakfast")),
                ("carb_grams".to_string(), json!(50.0)),
                ("protein_grams".to_string(), json!(40.0)),
                ("fat_grams".to_string(), json!(20.0)),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(36.0)),
            ]),
        });
        event_counter += 1;
        
        // Morning HIIT
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "exercise".to_string(),
            timestamp: wake_time + Duration::hours(2),
            end_timestamp: None,
            properties: HashMap::from([
                ("duration_minutes".to_string(), json!(45.0)),
                ("intensity".to_string(), json!("high_intensity")),
                ("type".to_string(), json!("hiit")),
                ("vo2max_percentage".to_string(), json!(85.0)),
            ]),
        });
        event_counter += 1;
        
        // Post-workout meal
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(3),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("post_workout")),
                ("carb_grams".to_string(), json!(60.0)),
                ("protein_grams".to_string(), json!(35.0)),
                ("fat_grams".to_string(), json!(10.0)),
                ("glycemic_index".to_string(), json!("medium")),
            ]),
        });
        event_counter += 1;
        
        // Afternoon steady cardio
        if day % 2 == 0 {
            events.push(Event {
                event_id: format!("evt_{}", event_counter),
                event_type: "exercise".to_string(),
                timestamp: wake_time + Duration::hours(8),
                end_timestamp: None,
                properties: HashMap::from([
                    ("duration_minutes".to_string(), json!(60.0)),
                    ("intensity".to_string(), json!("moderate")),
                    ("type".to_string(), json!("cardio")),
                    ("vo2max_percentage".to_string(), json!(65.0)),
                ]),
            });
            event_counter += 1;
        }
        
        // High-protein dinner
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(11),
            end_timestamp: None,
            properties: HashMap::from([
                ("meal_type".to_string(), json!("dinner")),
                ("carb_grams".to_string(), json!(55.0)),
                ("protein_grams".to_string(), json!(45.0)),
                ("fat_grams".to_string(), json!(20.0)),
                ("glycemic_index".to_string(), json!("low")),
                ("protein_percentage".to_string(), json!(38.0)),
            ]),
        });
        event_counter += 1;
        
        // Good glucose control
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_blood_glucose".to_string(),
            timestamp: wake_time + Duration::hours(12),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(92.0)),
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
        
        // Rotating sleep schedule
        let sleep_hour = if day % 3 == 0 {
            23 // Normal night
        } else if day % 3 == 1 {
            8 // Morning sleep (night shift)
        } else {
            15 // Afternoon sleep (evening shift)
        };
        
        let sleep_start = day_start.with_hour(sleep_hour).unwrap().with_minute(0).unwrap();
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "sleep".to_string(),
            timestamp: sleep_start,
            end_timestamp: Some(sleep_start + Duration::hours(6)),
            properties: HashMap::from([
                ("duration_hours".to_string(), json!(6.0)),
                ("quality".to_string(), json!(if day % 3 == 0 { "fair" } else { "poor" })),
                ("sleep_efficiency".to_string(), json!(if day % 3 == 0 { 0.75 } else { 0.62 })),
            ]),
        });
        event_counter += 1;
        
        let wake_time = sleep_start + Duration::hours(6);
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "wake".to_string(),
            timestamp: wake_time,
            end_timestamp: None,
            properties: HashMap::new(),
        });
        event_counter += 1;
        
        // Variable HRV
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "health_hrv".to_string(),
            timestamp: wake_time + Duration::minutes(15),
            end_timestamp: None,
            properties: HashMap::from([
                ("value".to_string(), json!(if day % 3 == 0 { 55.0 } else { 38.0 })),
                ("unit".to_string(), json!("ms")),
            ]),
        });
        event_counter += 1;
        
        // High caffeine dependency
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: wake_time + Duration::minutes(30),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(200.0)),
            ]),
        });
        event_counter += 1;
        
        // Irregular meals
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "meal".to_string(),
            timestamp: wake_time + Duration::hours(2),
            end_timestamp: None,
            properties: HashMap::from([
                ("carb_grams".to_string(), json!(60.0)),
                ("protein_grams".to_string(), json!(20.0)),
                ("fat_grams".to_string(), json!(18.0)),
                ("glycemic_index".to_string(), json!("medium")),
            ]),
        });
        event_counter += 1;
        
        // Another caffeine dose
        events.push(Event {
            event_id: format!("evt_{}", event_counter),
            event_type: "caffeine".to_string(),
            timestamp: wake_time + Duration::hours(6),
            end_timestamp: None,
            properties: HashMap::from([
                ("dose_mg".to_string(), json!(150.0)),
            ]),
        });
        event_counter += 1;
        
        // Circadian misalignment - inappropriate light exposure
        if day % 3 != 0 {
            events.push(Event {
                event_id: format!("evt_{}", event_counter),
                event_type: "light_exposure".to_string(),
                timestamp: wake_time + Duration::hours(4),
                end_timestamp: None,
                properties: HashMap::from([
                    ("intensity_lux".to_string(), json!(3000.0)),
                    ("duration_minutes".to_string(), json!(60.0)),
                ]),
            });
            event_counter += 1;
        }
    }
    
    events
}