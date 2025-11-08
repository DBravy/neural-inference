// cli.rs - Command-line interface for neural primitive estimation
use neurological_primitives::{EventData, Primitive, PrimitiveEstimator};
use chrono::{DateTime, Utc};
use std::fs;

fn main() {
    let json_data = fs::read_to_string("mock_data.json").expect("Unable to read file");
    let event_data: EventData = serde_json::from_str(&json_data).expect("Unable to parse JSON");
    
    let estimator = PrimitiveEstimator::new();
    
    // Use a timestamp that aligns with the mock data (Jan 18, 2025, 10:00 AM UTC)
    let estimation_time = DateTime::parse_from_rfc3339("2025-01-18T10:00:00Z")
        .expect("Failed to parse estimation time")
        .with_timezone(&Utc);
    let result = estimator.estimate_at_time(&event_data.events, estimation_time);
    
    print_result(&result);
}

fn print_result(result: &neurological_primitives::EstimationResult) {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║           NEUROBIOLOGICAL PRIMITIVE ESTIMATION RESULTS          ║");
    println!("║                  WITH PHYSIOLOGICAL VALIDATION                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    println!("Timestamp: {}\n", result.timestamp);

    // Print physiological constraints if any were applied
    if !result.physiological_constraints.is_empty() {
        println!("\n╔══════════════════════════════════════════════════════════════════╗");
        println!("║              PHYSIOLOGICAL VALIDATION ADJUSTMENTS                ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
        
        for constraint in &result.physiological_constraints {
            println!("🔬 {} → {}", constraint.constraint_source, constraint.primitive);
            println!("   Original Score: {:.3} → Adjusted: {:.3}", 
                constraint.original_score, constraint.adjusted_score);
            if constraint.confidence_impact != 0.0 {
                println!("   Confidence Impact: {:+.2}", constraint.confidence_impact);
            }
            println!("   Reason: {}", constraint.reason);
            println!();
        }
    }

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                     PRIMITIVE ESTIMATES                          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    for primitive in Primitive::all() {
        let key = primitive.as_str();
        if let Some(state) = result.primitives.get(key) {
            let confidence_indicator = if state.confidence < 0.6 {
                "⚠️ "
            } else if state.confidence > 0.9 {
                "✓ "
            } else {
                ""
            };
            
            println!("┌─ {} {}", key.to_uppercase(), confidence_indicator);
            println!("│  Score: {:.3} (Confidence: {:.1}%)", 
                state.modified_score, state.confidence * 100.0);
            
            if let Some(acute) = state.acute_score {
                println!("│  Acute: {:.3} | Chronic: {:.3}", 
                    acute, state.chronic_score.unwrap_or(0.0));
            }
            if let Some(effective) = state.effective_score {
                println!("│  Effective (after inhibition): {:.3}", effective);
            }
            
            println!("│  Description: {}", get_level_description(key, state.modified_score));
            
            if !state.contributors.is_empty() {
                println!("│");
                println!("│  Top Contributors:");
                for (i, contrib) in state.contributors.iter().enumerate().take(3) {
                    let sign = if contrib.decayed_impact >= 0.0 { "+" } else { "" };
                    println!("│    {}. {} ({:.1}h ago): {}{:.3}",
                        i + 1,
                        contrib.event_type,
                        contrib.hours_ago,
                        sign,
                        contrib.decayed_impact
                    );
                }
            }
            println!("└─────────────────────────────────────────────────────────");
            println!();
        }
    }

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                  SLEEP DRIVE (Two-Process Model)                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    let sleep_status = if result.sleep_drive >= 0.75 {
        "🔴 VERY HIGH - Strong urge to sleep"
    } else if result.sleep_drive >= 0.6 {
        "🟠 HIGH - Significant sleep pressure"
    } else if result.sleep_drive >= 0.4 {
        "🟡 MODERATE - Building sleep pressure"
    } else {
        "🟢 LOW - Alert and wakeful"
    };
    
    println!("Overall Sleep Drive: {:.3} ({})", result.sleep_drive, sleep_status);
    println!();

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                      INTERPRETATION                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Functional State: {}", result.functional_state.state_type);
    println!("{}", result.functional_state.description);
    println!("\nRecommendations:");
    for (i, rec) in result.functional_state.recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, rec);
    }
}

fn get_level_description(primitive_key: &str, score: f64) -> String {
    match primitive_key {
        "dopamine" => {
            if score >= 0.7 {
                "High (good for focus/work)".to_string()
            } else if score >= 0.5 {
                "Moderate".to_string()
            } else if score >= 0.3 {
                "Low (may affect motivation)".to_string()
            } else {
                "Very low (impaired motivation/focus)".to_string()
            }
        },
        "serotonin" => {
            if score >= 0.7 {
                "High (stable mood)".to_string()
            } else if score >= 0.5 {
                "Moderate".to_string()
            } else if score >= 0.3 {
                "Low (may affect mood)".to_string()
            } else {
                "Very low (mood instability risk)".to_string()
            }
        },
        "norepinephrine" => {
            if score >= 0.7 {
                "High (alert and focused)".to_string()
            } else if score >= 0.5 {
                "Moderate".to_string()
            } else if score >= 0.3 {
                "Low (reduced alertness)".to_string()
            } else {
                "Very low (drowsy)".to_string()
            }
        },
        "cortisol" => {
            if score >= 0.7 {
                "High (stressed/activated)".to_string()
            } else if score >= 0.5 {
                "Moderate (normal stress response)".to_string()
            } else if score >= 0.3 {
                "Low (relaxed)".to_string()
            } else {
                "Very low (calm/depleted)".to_string()
            }
        },
        "adenosine" => {
            if score >= 0.7 {
                "High pressure (need sleep)".to_string()
            } else if score >= 0.5 {
                "Moderate pressure (building)".to_string()
            } else if score >= 0.3 {
                "Low pressure (alert)".to_string()
            } else {
                "Very low pressure (recently rested)".to_string()
            }
        },
        "glucose" => {
            if score >= 0.7 {
                "High (good energy availability)".to_string()
            } else if score >= 0.5 {
                "Moderate".to_string()
            } else if score >= 0.3 {
                "Low (may need food)".to_string()
            } else {
                "Very low (depleted)".to_string()
            }
        },
        _ => format!("{:.3}", score)
    }
}

