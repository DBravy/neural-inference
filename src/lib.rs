// lib.rs - Research-Based Neurobiological Primitive Estimator Library
pub mod profiles;
pub mod chat;

// Re-export profile functions for convenience
pub use profiles::{generate_profile_events, get_all_profiles, Profile};

use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventData {
    pub user_id: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<DateTime<Utc>>,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Primitive {
    Dopamine,
    Norepinephrine,
    Serotonin,
    Adenosine,
    CircadianPhase,
    Cortisol,
    Glucose,
}

impl Primitive {
    pub fn as_str(&self) -> &str {
        match self {
            Primitive::Dopamine => "dopamine",
            Primitive::Norepinephrine => "norepinephrine",
            Primitive::Serotonin => "serotonin",
            Primitive::Adenosine => "adenosine",
            Primitive::CircadianPhase => "circadian_phase",
            Primitive::Cortisol => "cortisol",
            Primitive::Glucose => "glucose",
        }
    }

    pub fn all() -> Vec<Primitive> {
        vec![
            Primitive::Dopamine,
            Primitive::Norepinephrine,
            Primitive::Serotonin,
            Primitive::Adenosine,
            Primitive::CircadianPhase,
            Primitive::Cortisol,
            Primitive::Glucose,
        ]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PrimitiveState {
    pub base_score: f64,
    pub modified_score: f64,
    pub contributors: Vec<EventContribution>,
    pub confidence: f64,  // NEW: Confidence score (0.0-1.0)
    // For dopamine and serotonin only
    pub acute_score: Option<f64>,      // Recent state (8-12h window)
    pub chronic_score: Option<f64>,    // Baseline state (72h+ window)
    pub effective_score: Option<f64>,  // After reciprocal inhibition
}

#[derive(Debug, Clone, Serialize)]
pub struct EventContribution {
    pub event_id: String,
    pub event_type: String,
    pub impact: f64,
    pub decayed_impact: f64,
    pub hours_ago: f64,
}

#[derive(Debug, Serialize)]
pub struct EstimationResult {
    pub timestamp: DateTime<Utc>,
    pub primitives: HashMap<String, PrimitiveState>,
    pub detected_sequences: Vec<DetectedSequence>,
    pub sleep_drive: f64,
    pub dopamine_serotonin_ratio: f64,
    pub functional_state: FunctionalState,
    pub physiological_constraints: Vec<PhysiologicalConstraintApplied>,  // NEW: Track applied constraints
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionalState {
    pub state_type: String,
    pub description: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DetectedSequence {
    pub pattern_name: String,
    pub events: Vec<String>,
    pub impact_on_primitive: String,
    pub adjustment: f64,
}

// ============================================================================
// PHYSIOLOGICAL VALIDATION STRUCTURES
// ============================================================================

#[derive(Debug, Clone)]
pub struct PhysiologicalMeasurement {
    pub measurement_type: MeasurementType,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementType {
    HeartRate,
    HeartRateVariability,
    BloodOxygen,
    BloodGlucose,
    BodyTemperature,
    RespiratoryRate,
    Steps,
}

#[derive(Debug, Clone)]
pub enum ConstraintType {
    Floor(f64),              // Minimum value for primitive
    Ceiling(f64),            // Maximum value for primitive
    Override(f64),           // Strong signal, replace estimate
    ConfidencePenalty(f64),  // Reduce confidence in estimate
}

#[derive(Debug, Clone)]
pub struct PhysiologicalConstraint {
    pub source_measurement: MeasurementType,
    pub source_value: f64,
    pub affects_primitive: Primitive,
    pub constraint_type: ConstraintType,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhysiologicalConstraintApplied {
    pub primitive: String,
    pub constraint_source: String,
    pub original_score: f64,
    pub adjusted_score: f64,
    pub confidence_impact: f64,
    pub reason: String,
}

// ============================================================================
// CONTEXT WINDOWS
// ============================================================================

pub struct ContextConfig {
    pub window_hours: i64,
    pub decay_half_life_hours: f64,
}

impl ContextConfig {
    pub fn for_primitive(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Glucose => ContextConfig {
                window_hours: 8,
                decay_half_life_hours: 2.0,
            },
            Primitive::Norepinephrine => ContextConfig {
                window_hours: 12,
                decay_half_life_hours: 4.0,
            },
            Primitive::Adenosine => ContextConfig {
                window_hours: 20,
                decay_half_life_hours: 16.0,
            },
            Primitive::Cortisol => ContextConfig {
                window_hours: 48,
                decay_half_life_hours: 12.0,
            },
            Primitive::Dopamine => ContextConfig {
                window_hours: 72,
                decay_half_life_hours: 24.0,
            },
            Primitive::Serotonin => ContextConfig {
                window_hours: 96,
                decay_half_life_hours: 36.0,
            },
            Primitive::CircadianPhase => ContextConfig {
                window_hours: 168,
                decay_half_life_hours: 72.0,
            },
        }
    }
    
    pub fn acute_for_monoamine(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Dopamine => ContextConfig {
                window_hours: 12,
                decay_half_life_hours: 6.0,
            },
            Primitive::Serotonin => ContextConfig {
                window_hours: 16,
                decay_half_life_hours: 8.0,
            },
            _ => Self::for_primitive(primitive),
        }
    }
    
    pub fn chronic_for_monoamine(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Dopamine => ContextConfig {
                window_hours: 72,
                decay_half_life_hours: 24.0,
            },
            Primitive::Serotonin => ContextConfig {
                window_hours: 96,
                decay_half_life_hours: 36.0,
            },
            _ => Self::for_primitive(primitive),
        }
    }
}

// ============================================================================
// PHYSIOLOGICAL CONSTRAINT GENERATION
// ============================================================================

impl PhysiologicalMeasurement {
    /// Generate constraints based on this measurement's value
    pub fn generate_constraints(&self, estimation_time: DateTime<Utc>) -> Vec<PhysiologicalConstraint> {
        let mut constraints = Vec::new();
        
        // Only apply constraints for recent measurements (within 4 hours for acute signals)
        let hours_ago = (estimation_time - self.timestamp).num_minutes() as f64 / 60.0;
        
        match self.measurement_type {
            MeasurementType::HeartRateVariability => {
                self.generate_hrv_constraints(&mut constraints, hours_ago);
            },
            MeasurementType::HeartRate => {
                self.generate_hr_constraints(&mut constraints, hours_ago);
            },
            MeasurementType::BloodOxygen => {
                self.generate_spo2_constraints(&mut constraints, hours_ago);
            },
            MeasurementType::BloodGlucose => {
                self.generate_glucose_constraints(&mut constraints, hours_ago);
            },
            MeasurementType::BodyTemperature => {
                self.generate_temperature_constraints(&mut constraints, hours_ago);
            },
            MeasurementType::RespiratoryRate => {
                self.generate_respiratory_constraints(&mut constraints, hours_ago);
            },
            MeasurementType::Steps => {
                self.generate_steps_constraints(&mut constraints, hours_ago);
            },
        }
        
        constraints
    }
    
    fn generate_hrv_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // HRV (RMSSD in ms) - only apply for recent measurements (< 2 hours)
        if hours_ago > 2.0 {
            return;
        }
        
        let rmssd = self.value;
        
        // Very low HRV (<30ms) indicates high sympathetic activation
        if rmssd < 30.0 {
            // High cortisol floor
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::HeartRateVariability,
                source_value: rmssd,
                affects_primitive: Primitive::Cortisol,
                constraint_type: ConstraintType::Floor(0.6),
                reason: format!("Very low HRV ({:.1}ms) indicates high stress/cortisol", rmssd),
            });
            
            // Low adenosine confidence penalty (HRV suppressed by stress, not just sleep debt)
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::HeartRateVariability,
                source_value: rmssd,
                affects_primitive: Primitive::Adenosine,
                constraint_type: ConstraintType::ConfidencePenalty(0.7),
                reason: format!("Low HRV ({:.1}ms) suggests stress rather than pure sleep debt", rmssd),
            });
        }
        
        // High HRV (>70ms) indicates good parasympathetic tone
        if rmssd > 70.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::HeartRateVariability,
                source_value: rmssd,
                affects_primitive: Primitive::Cortisol,
                constraint_type: ConstraintType::Ceiling(0.4),
                reason: format!("High HRV ({:.1}ms) indicates low stress/cortisol", rmssd),
            });
        }
    }
    
    fn generate_hr_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // Heart rate (BPM) - only apply for recent measurements (< 1 hour)
        if hours_ago > 1.0 {
            return;
        }
        
        let hr = self.value;
        
        // Elevated resting HR (>80 bpm) suggests sympathetic activation
        if hr > 80.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::HeartRate,
                source_value: hr,
                affects_primitive: Primitive::Norepinephrine,
                constraint_type: ConstraintType::Floor(0.5),
                reason: format!("Elevated HR ({:.0} bpm) indicates norepinephrine activity", hr),
            });
            
            // High HR with predicted high adenosine is a contradiction
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::HeartRate,
                source_value: hr,
                affects_primitive: Primitive::Adenosine,
                constraint_type: ConstraintType::ConfidencePenalty(0.6),
                reason: format!("High HR ({:.0} bpm) contradicts high sleep pressure", hr),
            });
        }
        
        // Very low resting HR (<55 bpm) in non-athletes suggests low arousal
        if hr < 55.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::HeartRate,
                source_value: hr,
                affects_primitive: Primitive::Norepinephrine,
                constraint_type: ConstraintType::Ceiling(0.4),
                reason: format!("Low HR ({:.0} bpm) indicates low arousal/norepinephrine", hr),
            });
        }
    }
    
    fn generate_spo2_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // Blood oxygen (SpO2 %) - mainly relevant for recent sleep quality
        let spo2 = self.value;
        
        // Low SpO2 during sleep indicates poor sleep quality
        if spo2 < 92.0 && hours_ago < 8.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::BloodOxygen,
                source_value: spo2,
                affects_primitive: Primitive::Dopamine,
                constraint_type: ConstraintType::ConfidencePenalty(0.5),
                reason: format!("Low SpO2 ({:.1}%) suggests impaired sleep quality affecting dopamine recovery", spo2),
            });
        }
    }
    
    fn generate_glucose_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // Blood glucose (mg/dL) - only apply for recent measurements (< 2 hours)
        if hours_ago > 2.0 {
            return;
        }
        
        let glucose_mg_dl = self.value;
        
        // Hypoglycemia (<70 mg/dL) triggers cortisol release
        if glucose_mg_dl < 70.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::BloodGlucose,
                source_value: glucose_mg_dl,
                affects_primitive: Primitive::Cortisol,
                constraint_type: ConstraintType::Floor(0.6),
                reason: format!("Hypoglycemia ({:.0} mg/dL) triggers cortisol release", glucose_mg_dl),
            });
            
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::BloodGlucose,
                source_value: glucose_mg_dl,
                affects_primitive: Primitive::Glucose,
                constraint_type: ConstraintType::Override(0.2),
                reason: format!("Direct glucose measurement: {:.0} mg/dL (low)", glucose_mg_dl),
            });
        }
        
        // Normal glucose (70-100 mg/dL)
        if glucose_mg_dl >= 70.0 && glucose_mg_dl <= 100.0 {
            let normalized_score = 0.5 + ((glucose_mg_dl - 85.0) / 30.0).clamp(-0.3, 0.3);
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::BloodGlucose,
                source_value: glucose_mg_dl,
                affects_primitive: Primitive::Glucose,
                constraint_type: ConstraintType::Override(normalized_score),
                reason: format!("Direct glucose measurement: {:.0} mg/dL (normal)", glucose_mg_dl),
            });
        }
        
        // Hyperglycemia (>140 mg/dL)
        if glucose_mg_dl > 140.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::BloodGlucose,
                source_value: glucose_mg_dl,
                affects_primitive: Primitive::Glucose,
                constraint_type: ConstraintType::Override(0.8),
                reason: format!("Direct glucose measurement: {:.0} mg/dL (high)", glucose_mg_dl),
            });
        }
    }
    
    fn generate_temperature_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // Body temperature (°C) - circadian phase marker
        // Core body temp minimum typically 2-3h before wake
        let temp_c = self.value;
        
        // Abnormally low temperature suggests circadian nadir
        if temp_c < 36.5 && hours_ago < 4.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::BodyTemperature,
                source_value: temp_c,
                affects_primitive: Primitive::CircadianPhase,
                constraint_type: ConstraintType::ConfidencePenalty(0.8),
                reason: format!("Low body temp ({:.1}°C) indicates circadian nadir timing", temp_c),
            });
        }
    }
    
    fn generate_respiratory_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // Respiratory rate (breaths per minute) - only apply for recent measurements
        if hours_ago > 1.0 {
            return;
        }
        
        let rr = self.value;
        
        // Elevated respiratory rate (>18 bpm) suggests anxiety/stress
        if rr > 18.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::RespiratoryRate,
                source_value: rr,
                affects_primitive: Primitive::Cortisol,
                constraint_type: ConstraintType::Floor(0.5),
                reason: format!("Elevated respiratory rate ({:.0} bpm) indicates stress", rr),
            });
            
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::RespiratoryRate,
                source_value: rr,
                affects_primitive: Primitive::Serotonin,
                constraint_type: ConstraintType::ConfidencePenalty(0.7),
                reason: format!("High respiratory rate ({:.0} bpm) suggests anxiety/low serotonin", rr),
            });
        }
    }
    
    fn generate_steps_constraints(&self, constraints: &mut Vec<PhysiologicalConstraint>, hours_ago: f64) {
        // Daily step count - only check if looking at a full day window
        if hours_ago < 12.0 || hours_ago > 30.0 {
            return;
        }
        
        let steps = self.value;
        
        // Very low activity (<2000 steps/day) with predicted high dopamine is contradictory
        if steps < 2000.0 {
            constraints.push(PhysiologicalConstraint {
                source_measurement: MeasurementType::Steps,
                source_value: steps,
                affects_primitive: Primitive::Dopamine,
                constraint_type: ConstraintType::ConfidencePenalty(0.6),
                reason: format!("Very low activity ({:.0} steps) contradicts high dopamine predictions", steps),
            });
        }
    }
}

// ============================================================================
// RESEARCH-BASED IMPACT COMPUTATION FUNCTIONS (continued from original)
// ============================================================================

/// Compute impacts of a sleep event on all primitives
fn compute_sleep_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let duration = event.properties.get("duration_hours")
        .and_then(|v| v.as_f64())
        .unwrap_or(7.0);
    
    let quality_str = event.properties.get("quality")
        .and_then(|v| v.as_str())
        .unwrap_or("good");
    
    let quality_score = match quality_str {
        "excellent" => 1.0,
        "good" => 0.8,
        "fair" => 0.6,
        "poor" => 0.4,
        _ => 0.8,
    };
    
    let sleep_efficiency = event.properties.get("sleep_efficiency")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.85);
    
    let adenosine_clearance = -(duration / 7.5) * quality_score * 0.85;
    impacts.insert("adenosine".to_string(), adenosine_clearance);
    
    let dopamine_impact = if duration >= 7.0 {
        0.3 * quality_score
    } else if duration >= 6.0 {
        0.15 * quality_score
    } else {
        -0.2 * (1.0 - quality_score)
    };
    impacts.insert("dopamine".to_string(), dopamine_impact);
    
    let serotonin_impact = 0.25 * quality_score * (duration / 7.5).min(1.0);
    impacts.insert("serotonin".to_string(), serotonin_impact);
    
    // Adjusted to work with circadian baseline system
    // Good sleep provides modest cortisol reduction (within -0.15 relaxation cap)
    // Poor sleep increases cortisol, matching 37-45% evening elevation from research
    let cortisol_impact = if quality_score >= 0.7 {
        -0.12  // Reduced from -0.2 to stay within relaxation effect cap
    } else {
        0.15 * (1.0 - quality_score)  // Poor sleep impact remains same
    };
    impacts.insert("cortisol".to_string(), cortisol_impact);
    
    let glucose_impact = if duration >= 6.0 && sleep_efficiency >= 0.67 {
        0.2
    } else {
        -0.3 * (1.0 - (duration / 6.0).min(1.0))
    };
    impacts.insert("glucose".to_string(), glucose_impact);
    
    let hour_of_sleep = event.timestamp.hour() as f64;
    let phase_impact = if hour_of_sleep >= 22.0 && hour_of_sleep <= 24.0 {
        0.0
    } else if hour_of_sleep >= 0.0 && hour_of_sleep <= 2.0 {
        0.2 * (hour_of_sleep / 2.0)
    } else {
        0.0
    };
    impacts.insert("circadian_phase".to_string(), phase_impact);
    
    impacts
}

fn compute_light_impacts(event: &Event, _estimation_time: DateTime<Utc>) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let intensity = event.properties.get("intensity_lux")
        .and_then(|v| v.as_f64())
        .unwrap_or(1000.0);
    
    let duration_min = event.properties.get("duration_minutes")
        .and_then(|v| v.as_f64())
        .unwrap_or(30.0);
    
    let hour_of_day = event.timestamp.hour() as f64;
    let is_morning = hour_of_day >= 6.0 && hour_of_day <= 11.0;
    let is_evening = hour_of_day >= 18.0 && hour_of_day <= 22.0;
    
    let circadian_impact = if intensity >= 2000.0 {
        if is_morning {
            -0.8 * (duration_min / 120.0).min(1.0)
        } else if is_evening {
            0.6 * (duration_min / 120.0).min(1.0)
        } else {
            -0.2
        }
    } else if intensity >= 100.0 {
        if is_morning {
            -0.3
        } else if is_evening {
            0.3
        } else {
            0.0
        }
    } else {
        0.0
    };
    impacts.insert("circadian_phase".to_string(), circadian_impact);
    
    if is_morning && intensity >= 2000.0 {
        let serotonin_impact = 0.25 * (intensity / 10000.0).min(1.0) * (duration_min / 30.0).min(1.0);
        impacts.insert("serotonin".to_string(), serotonin_impact);
    }
    
    if is_morning {
        // Morning light enhances CAR, but the awakening_boost function already models this
        // Direct cortisol impact should be modest to avoid double-counting
        let cortisol_impact = if intensity >= 5000.0 {
            0.08  // Reduced from 0.2
        } else if intensity >= 800.0 {
            0.05  // Reduced from 0.15
        } else {
            0.02  // Reduced from 0.05
        };
        impacts.insert("cortisol".to_string(), cortisol_impact);
    }
    
    impacts
}

fn compute_meal_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let carbs = event.properties.get("carb_grams")
        .and_then(|v| v.as_f64())
        .unwrap_or(50.0);
    
    let protein = event.properties.get("protein_grams")
        .and_then(|v| v.as_f64())
        .unwrap_or(20.0);
    
    let protein_pct = event.properties.get("protein_percentage")
        .and_then(|v| v.as_f64())
        .unwrap_or(20.0);
    
    let gi_str = event.properties.get("glycemic_index")
        .and_then(|v| v.as_str())
        .unwrap_or("medium");
    
    let gi_score = match gi_str {
        "low" => 0.3,
        "medium" => 0.6,
        "high" => 1.0,
        _ => 0.6,
    };

    let glucose_impact = if protein >= 50.0 {
        0.1 + gi_score * 0.1  // High protein: 0.1-0.2
    } else {
        0.15 + gi_score * 0.15  // Normal: 0.15-0.3
    };
    impacts.insert("glucose".to_string(), glucose_impact);
    
    let serotonin_impact = if protein_pct < 10.0 && carbs > 40.0 {
        0.35 * (1.0 - protein_pct / 20.0).max(0.0)
    } else if protein_pct > 25.0 {
        -0.15
    } else {
        0.1
    };
    impacts.insert("serotonin".to_string(), serotonin_impact);
    
    let dopamine_impact = if protein >= 15.0 {
        0.15 * (protein / 50.0).min(1.0)
    } else {
        0.05
    };
    let gi_dopamine_boost = gi_score * 0.1;
    impacts.insert("dopamine".to_string(), dopamine_impact + gi_dopamine_boost);
    
    let meal_type = event.properties.get("meal_type")
        .and_then(|v| v.as_str())
        .unwrap_or("lunch");
    
    let hour_of_meal = event.timestamp.hour() as f64;
    let circadian_impact = match meal_type {
        "breakfast" if hour_of_meal >= 6.0 && hour_of_meal <= 9.0 => 0.1,
        "dinner" if hour_of_meal >= 18.0 && hour_of_meal <= 20.0 => 0.0,
        "dinner" if hour_of_meal >= 21.0 => 0.2,
        _ => 0.0,
    };
    impacts.insert("circadian_phase".to_string(), circadian_impact);
    
    impacts
}

fn compute_caffeine_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let dose_mg = event.properties.get("dose_mg")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);
    
    let a2a_occupancy = (dose_mg / (dose_mg + 65.0)).min(1.0);
    
    impacts.insert("adenosine".to_string(), -0.5 * a2a_occupancy);
    
    let dopamine_impact = 0.15 * (dose_mg / 200.0).min(1.0);
    impacts.insert("dopamine".to_string(), dopamine_impact);
    
    let norepinephrine_impact = 0.25 * (dose_mg / 200.0).min(1.3);
    impacts.insert("norepinephrine".to_string(), norepinephrine_impact);
    
    // Reduced from 0.2 to 0.15 to work with circadian baseline + stress sensitivity modulation
    // Research shows ~50% increase, but this is on top of already-elevated morning baseline
    let cortisol_impact = 0.15 * (dose_mg / 200.0).min(1.0);
    impacts.insert("cortisol".to_string(), cortisol_impact);
    
    let hours_before_sleep = event.properties.get("hours_before_intended_sleep")
        .and_then(|v| v.as_f64());
    
    if let Some(hours) = hours_before_sleep {
        if hours <= 6.0 {
            let phase_delay = 0.3 * (dose_mg / 200.0).min(1.0) * (1.0 - hours / 6.0);
            impacts.insert("circadian_phase".to_string(), phase_delay);
        }
    }
    
    impacts
}

fn compute_exercise_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let duration_min = event.properties.get("duration_minutes")
        .and_then(|v| v.as_f64())
        .unwrap_or(30.0);
    
    let intensity_str = event.properties.get("intensity")
        .and_then(|v| v.as_str())
        .unwrap_or("moderate");
    
    let vo2max_pct = event.properties.get("vo2max_percentage")
        .and_then(|v| v.as_f64())
        .unwrap_or(65.0);
    
    let exercise_type = event.properties.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("cardio");
    
    let intensity_pct = match intensity_str {
        "light" => 40.0,
        "moderate" => 65.0,
        "vigorous" => 75.0,
        "high_intensity" => 85.0,
        _ => vo2max_pct,
    };
    
    let dopamine_boost = if exercise_type == "hiit" {
        0.35 * (duration_min / 45.0).min(1.0)
    } else if intensity_pct >= 70.0 {
        0.25 * (duration_min / 60.0).min(1.0)
    } else {
        0.15 * (duration_min / 60.0).min(1.0)
    };
    impacts.insert("dopamine".to_string(), dopamine_boost);
    
    let norepinephrine_boost = if intensity_pct >= 70.0 {
        0.4 * (intensity_pct / 100.0)
    } else {
        0.2
    };
    impacts.insert("norepinephrine".to_string(), norepinephrine_boost);
    
    let serotonin_boost = 0.2 * (duration_min / 60.0).min(1.0);
    impacts.insert("serotonin".to_string(), serotonin_boost);
    
    // High-intensity exercise spikes cortisol; moderate exercise reduces it
    // Adjusted to work with circadian baseline system
    let cortisol_impact = if intensity_pct >= 75.0 && duration_min >= 45.0 {
        0.2  // Reduced from 0.3 for high-intensity
    } else {
        -0.08  // Reduced from -0.1 to work with relaxation cap
    };
    impacts.insert("cortisol".to_string(), cortisol_impact);
    
    let glucose_depletion = -0.3 * (intensity_pct / 100.0) * (duration_min / 60.0).min(1.0);
    impacts.insert("glucose".to_string(), glucose_depletion);
    
    impacts
}

fn compute_nap_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let duration_min = event.properties.get("duration_minutes")
        .and_then(|v| v.as_f64())
        .unwrap_or(20.0);
    
    let adenosine_clearance = if duration_min <= 30.0 {
        -0.25 * (duration_min / 30.0)
    } else {
        -0.4 * (duration_min / 90.0).min(1.0)
    };
    impacts.insert("adenosine".to_string(), adenosine_clearance);
    
    if duration_min >= 60.0 {
        impacts.insert("circadian_phase".to_string(), 0.15);
    }
    
    impacts
}

fn compute_stress_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let intensity_str = event.properties.get("intensity")
        .and_then(|v| v.as_str())
        .unwrap_or("moderate");
    
    let intensity_score = match intensity_str {
        "mild" => 0.3,
        "moderate" => 0.6,
        "high" => 1.0,
        "severe" => 1.3,
        _ => 0.6,
    };
    
    let controllable = event.properties.get("controllable")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let social_evaluative = event.properties.get("social_evaluative")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let mut cortisol_multiplier = 1.0;
    if !controllable {
        cortisol_multiplier *= 1.4;  // Reduced from 1.5
    }
    if social_evaluative {
        cortisol_multiplier *= 1.25; // Reduced from 1.3
    }
    
    // Reduced base impact from 0.4 to 0.25 to work with circadian baseline system
    // The circadian stress sensitivity (0.5-1.0) and awakening boost (1.0-1.75) 
    // will further modulate this, so the raw impact should be more modest
    let cortisol_impact: f64 = 0.25 * intensity_score * cortisol_multiplier;
    impacts.insert("cortisol".to_string(), cortisol_impact.min(0.55));  // Cap at 0.55 instead of 0.8
    
    let norepinephrine_impact = 0.3 * intensity_score;
    impacts.insert("norepinephrine".to_string(), norepinephrine_impact);
    
    let dopamine_impact = -0.2 * intensity_score;
    impacts.insert("dopamine".to_string(), dopamine_impact);
    
    let serotonin_impact = -0.25 * intensity_score;
    impacts.insert("serotonin".to_string(), serotonin_impact);
    
    // Stress triggers glucose release via cortisol and catecholamines (counterregulatory response)
    // Scales with stress intensity and cortisol response
    let glucose_impact = 0.25 * intensity_score * (cortisol_multiplier / 2.0).min(1.2);
    impacts.insert("glucose".to_string(), glucose_impact.min(0.4));
    
    impacts
}

fn compute_social_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let quality_str = event.properties.get("quality")
        .and_then(|v| v.as_str())
        .unwrap_or("neutral");
    
    let quality_score = match quality_str {
        "very_positive" => 1.0,
        "positive" => 0.7,
        "neutral" => 0.0,
        "negative" => -0.5,
        "very_negative" => -1.0,
        _ => 0.0,
    };
    
    let duration_hours = event.properties.get("duration_minutes")
        .and_then(|v| v.as_f64())
        .unwrap_or(60.0) / 60.0;
    
    if quality_score > 0.0 {
        let serotonin_boost = 0.3 * quality_score * (duration_hours / 2.0).min(1.0);
        impacts.insert("serotonin".to_string(), serotonin_boost);
        
        let dopamine_boost = 0.2 * quality_score;
        impacts.insert("dopamine".to_string(), dopamine_boost);
        
        impacts.insert("cortisol".to_string(), -0.2 * quality_score);
    } else {
        impacts.insert("serotonin".to_string(), 0.3 * quality_score);
        impacts.insert("cortisol".to_string(), -0.4 * quality_score);
    }
    
    impacts
}

fn compute_screen_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let hours_before_sleep = event.properties.get("hours_before_sleep")
        .and_then(|v| v.as_f64());
    
    let blue_light_str = event.properties.get("blue_light_intensity")
        .and_then(|v| v.as_str())
        .unwrap_or("medium");
    
    if let Some(hours) = hours_before_sleep {
        if hours <= 3.0 {
            let blue_light_factor = match blue_light_str {
                "low" => 0.3,
                "medium" => 0.6,
                "high" => 1.0,
                _ => 0.6,
            };
            
            let phase_delay = 0.25 * blue_light_factor * (1.0 - hours / 3.0);
            impacts.insert("circadian_phase".to_string(), phase_delay);
            
            let adenosine_suppression = -0.15 * blue_light_factor;
            impacts.insert("adenosine".to_string(), adenosine_suppression);
        }
    }
    
    impacts
}

fn compute_interruption_impacts(event: &Event) -> HashMap<String, f64> {
    let mut impacts = HashMap::new();
    
    let frequency = event.properties.get("frequency")
        .and_then(|v| v.as_f64())
        .unwrap_or(3.0);
    
    let _disruption_min = event.properties.get("total_disruption_minutes")
        .and_then(|v| v.as_f64())
        .unwrap_or(15.0);
    
    let stress_factor = (frequency / 10.0).min(1.0);
    
    impacts.insert("cortisol".to_string(), 0.2 * stress_factor);
    impacts.insert("dopamine".to_string(), -0.15 * stress_factor);
    
    impacts
}

// ============================================================================
// PRIMITIVE ESTIMATOR (continued)
// ============================================================================

pub struct PrimitiveEstimator {
    baseline_values: HashMap<String, f64>,
    current_adenosine_level: Cell<f64>,
}

impl PrimitiveEstimator {
    pub fn new() -> Self {
        let mut baseline_values = HashMap::new();
        
        baseline_values.insert("dopamine".to_string(), 0.5);
        baseline_values.insert("norepinephrine".to_string(), 0.5);
        baseline_values.insert("serotonin".to_string(), 0.5);
        baseline_values.insert("adenosine".to_string(), 0.3);
        baseline_values.insert("circadian_phase".to_string(), 0.5);
        baseline_values.insert("cortisol".to_string(), 0.4);
        baseline_values.insert("glucose".to_string(), 0.5);

        PrimitiveEstimator {
            baseline_values,
            current_adenosine_level: Cell::new(0.3),
        }
    }

    /// Main estimation function with physiological validation
    pub fn estimate_at_time(
        &self,
        events: &[Event],
        estimation_time: DateTime<Utc>,
    ) -> EstimationResult {
        println!("\n=== Estimating Primitives at {} ===", estimation_time);

        // PASS 1: Compute base scores from behavioral events
        let mut base_scores = HashMap::new();
        let mut all_contributors = HashMap::new();
        let mut acute_scores = HashMap::new();
        let mut chronic_scores = HashMap::new();

        // Compute adenosine first (needed for circadian gating)
        let (adenosine_score, adenosine_contributors) = self.compute_base_score(
            Primitive::Adenosine,
            events,
            estimation_time,
        );
        base_scores.insert("adenosine".to_string(), adenosine_score);
        all_contributors.insert("adenosine".to_string(), adenosine_contributors);
        self.current_adenosine_level.set(adenosine_score);
        
        // Then compute circadian phase
        let (circadian_score, circadian_contributors) = self.compute_base_score(
            Primitive::CircadianPhase,
            events,
            estimation_time,
        );
        base_scores.insert("circadian_phase".to_string(), circadian_score);
        all_contributors.insert("circadian_phase".to_string(), circadian_contributors);
        
        // Compute dopamine and serotonin with acute/chronic breakdown
        let (da_acute, da_chronic, da_combined, da_contributors) = 
            self.compute_monoamine_scores(Primitive::Dopamine, events, estimation_time);
        base_scores.insert("dopamine".to_string(), da_combined);
        all_contributors.insert("dopamine".to_string(), da_contributors);
        acute_scores.insert("dopamine".to_string(), da_acute);
        chronic_scores.insert("dopamine".to_string(), da_chronic);
        
        let (ser_acute, ser_chronic, ser_combined, ser_contributors) = 
            self.compute_monoamine_scores(Primitive::Serotonin, events, estimation_time);
        base_scores.insert("serotonin".to_string(), ser_combined);
        all_contributors.insert("serotonin".to_string(), ser_contributors);
        acute_scores.insert("serotonin".to_string(), ser_acute);
        chronic_scores.insert("serotonin".to_string(), ser_chronic);
        
        // Compute remaining primitives
        for primitive in Primitive::all() {
            let key = primitive.as_str();
            if key == "adenosine" || key == "circadian_phase" || key == "dopamine" || key == "serotonin" {
                continue;
            }
            
            let (score, contributors) = self.compute_base_score(primitive, events, estimation_time);
            base_scores.insert(key.to_string(), score);
            all_contributors.insert(key.to_string(), contributors);
        }

        // Compute sleep drive
        let sleep_drive = self.compute_sleep_drive(
            adenosine_score,
            circadian_score,
            estimation_time,
        );

        // PASS 2: Detect sequences
        let detected_sequences = self.detect_sequences(events, estimation_time);

        // PASS 3: Apply sequence adjustments
        let mut adjusted_scores = base_scores.clone();
        for sequence in &detected_sequences {
            if let Some(score) = adjusted_scores.get_mut(&sequence.impact_on_primitive) {
                *score = (*score + sequence.adjustment).clamp(0.0, 1.0);
            }
        }

        // PASS 4: Apply cross-primitive modifiers
        let (cross_modified_scores, effective_monoamines) = self.apply_cross_primitive_modifiers(&adjusted_scores);
        
        // PASS 5: PHYSIOLOGICAL VALIDATION LAYER (NEW)
        let (final_scores, confidence_scores, applied_constraints) = self.apply_physiological_validation(
            &cross_modified_scores,
            events,
            estimation_time,
        );
        
        // Get effective dopamine and serotonin after all modifications
        let dopamine_effective = effective_monoamines.get("dopamine").copied()
            .unwrap_or_else(|| final_scores.get("dopamine").copied().unwrap_or(0.5));
        let serotonin_effective = effective_monoamines.get("serotonin").copied()
            .unwrap_or_else(|| final_scores.get("serotonin").copied().unwrap_or(0.5));
        
        let da_ser_ratio = if serotonin_effective > 0.01 {
            dopamine_effective / serotonin_effective
        } else {
            dopamine_effective / 0.01
        };
        
        let functional_state = Self::compute_functional_state(dopamine_effective, serotonin_effective);

        // Build result with confidence scores
        let mut primitives = HashMap::new();
        for (key, base_score) in base_scores {
            let modified_score = final_scores.get(&key).copied().unwrap_or(base_score);
            let contributors = all_contributors.get(&key).cloned().unwrap_or_default();
            let confidence = confidence_scores.get(&key).copied().unwrap_or(1.0);
            
            let (acute_score, chronic_score, effective_score) = if key == "dopamine" || key == "serotonin" {
                (
                    acute_scores.get(&key).copied(),
                    chronic_scores.get(&key).copied(),
                    effective_monoamines.get(&key).copied(),
                )
            } else {
                (None, None, None)
            };
            
            primitives.insert(
                key,
                PrimitiveState {
                    base_score,
                    modified_score,
                    contributors,
                    confidence,
                    acute_score,
                    chronic_score,
                    effective_score,
                },
            );
        }

        EstimationResult {
            timestamp: estimation_time,
            primitives,
            detected_sequences,
            sleep_drive,
            dopamine_serotonin_ratio: da_ser_ratio,
            functional_state,
            physiological_constraints: applied_constraints,
        }
    }

    /// NEW: Apply physiological validation and constraints
    fn apply_physiological_validation(
        &self,
        scores: &HashMap<String, f64>,
        events: &[Event],
        estimation_time: DateTime<Utc>,
    ) -> (HashMap<String, f64>, HashMap<String, f64>, Vec<PhysiologicalConstraintApplied>) {
        // Extract physiological measurements from events
        let measurements = self.extract_physiological_measurements(events, estimation_time);
        
        if measurements.is_empty() {
            // No physiological data available, return original scores with full confidence
            let confidence_scores: HashMap<String, f64> = scores.keys()
                .map(|k| (k.clone(), 1.0))
                .collect();
            return (scores.clone(), confidence_scores, Vec::new());
        }
        
        // Generate constraints from all measurements
        let mut all_constraints = Vec::new();
        for measurement in &measurements {
            let constraints = measurement.generate_constraints(estimation_time);
            all_constraints.extend(constraints);
        }
        
        // Apply constraints to scores
        let mut adjusted_scores = scores.clone();
        let mut confidence_scores: HashMap<String, f64> = scores.keys()
            .map(|k| (k.clone(), 1.0))
            .collect();
        let mut applied_constraints = Vec::new();
        
        for constraint in all_constraints {
            let primitive_key = constraint.affects_primitive.as_str();
            if let Some(&current_score) = adjusted_scores.get(primitive_key) {
                let original_score = current_score;
                let mut new_score = current_score;
                let mut confidence_impact = 0.0;
                
                match &constraint.constraint_type {
                    ConstraintType::Floor(min_value) => {
                        if current_score < *min_value {
                            new_score = *min_value;
                            confidence_impact = -0.2; // Moderate confidence penalty for override
                        }
                    },
                    ConstraintType::Ceiling(max_value) => {
                        if current_score > *max_value {
                            new_score = *max_value;
                            confidence_impact = -0.2;
                        }
                    },
                    ConstraintType::Override(value) => {
                        // Strong signal - blend with original (70% measurement, 30% prediction)
                        new_score = value * 0.7 + current_score * 0.3;
                        confidence_impact = 0.3; // Increase confidence with direct measurement
                    },
                    ConstraintType::ConfidencePenalty(penalty) => {
                        // Don't change score, just reduce confidence
                        confidence_impact = -(1.0 - penalty);
                    },
                }
                
                // Apply changes
                adjusted_scores.insert(primitive_key.to_string(), new_score);
                
                if let Some(conf) = confidence_scores.get_mut(primitive_key) {
                    *conf = (*conf + confidence_impact).clamp(0.1, 1.0);
                }
                
                // Record what was applied
                if (new_score - original_score).abs() > 0.01 || confidence_impact.abs() > 0.01 {
                    applied_constraints.push(PhysiologicalConstraintApplied {
                        primitive: primitive_key.to_string(),
                        constraint_source: format!("{:?}", constraint.source_measurement),
                        original_score,
                        adjusted_score: new_score,
                        confidence_impact,
                        reason: constraint.reason.clone(),
                    });
                }
            }
        }
        
        (adjusted_scores, confidence_scores, applied_constraints)
    }
    
    /// Extract physiological measurements from health events
    fn extract_physiological_measurements(
        &self,
        events: &[Event],
        estimation_time: DateTime<Utc>,
    ) -> Vec<PhysiologicalMeasurement> {
        let mut measurements = Vec::new();
        
        // Look for health measurement events within a reasonable window
        let cutoff_time = estimation_time - Duration::hours(24);
        
        for event in events {
            if event.timestamp < cutoff_time || event.timestamp > estimation_time {
                continue;
            }
            
            let measurement_type = match event.event_type.as_str() {
                "health_heart_rate" => Some(MeasurementType::HeartRate),
                "health_hrv" => Some(MeasurementType::HeartRateVariability),
                "health_blood_oxygen" => Some(MeasurementType::BloodOxygen),
                "health_blood_glucose" => Some(MeasurementType::BloodGlucose),
                "health_body_temperature" => Some(MeasurementType::BodyTemperature),
                "health_respiratory_rate" => Some(MeasurementType::RespiratoryRate),
                "health_steps" => Some(MeasurementType::Steps),
                _ => None,
            };
            
            if let Some(mtype) = measurement_type {
                if let Some(value) = event.properties.get("value").and_then(|v| v.as_f64()) {
                    let unit = event.properties.get("unit")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    measurements.push(PhysiologicalMeasurement {
                        measurement_type: mtype,
                        value,
                        timestamp: event.timestamp,
                        unit,
                    });
                }
            }
        }
        
        measurements
    }

    fn compute_base_score(
        &self,
        primitive: Primitive,
        events: &[Event],
        estimation_time: DateTime<Utc>,
    ) -> (f64, Vec<EventContribution>) {
        let config = ContextConfig::for_primitive(primitive);
        let baseline = self.baseline_values
            .get(primitive.as_str())
            .copied()
            .unwrap_or(0.5);

        if primitive == Primitive::Adenosine {
            return self.compute_adenosine_special(events, estimation_time);
        }

        if primitive == Primitive::CircadianPhase {
            return self.compute_circadian_phase_special(events, estimation_time);
        }

        let cutoff_time = estimation_time - Duration::hours(config.window_hours);
        let relevant_events: Vec<_> = events
            .iter()
            .filter(|e| e.timestamp >= cutoff_time && e.timestamp <= estimation_time)
            .collect();

        let mut accumulated_impact = 0.0;
        let mut contributors = Vec::new();

        for event in relevant_events {
            let hours_ago = (estimation_time - event.timestamp).num_minutes() as f64 / 60.0;
            
            let impacts = self.compute_event_impacts(event, estimation_time);
            
            if let Some(&raw_impact) = impacts.get(primitive.as_str()) {
                let decay_factor = Self::exponential_decay(hours_ago, config.decay_half_life_hours);
                let decayed_impact = raw_impact * decay_factor;
                
                accumulated_impact += decayed_impact;
                
                contributors.push(EventContribution {
                    event_id: event.event_id.clone(),
                    event_type: event.event_type.clone(),
                    impact: raw_impact,
                    decayed_impact,
                    hours_ago,
                });
            }
        }

        if primitive == Primitive::Dopamine {
            let caffeine_boost = self.compute_caffeine_dopamine_boost(events, estimation_time);
            accumulated_impact += caffeine_boost;
        }

        

        contributors.sort_by(|a, b| {
            b.decayed_impact
                .abs()
                .partial_cmp(&a.decayed_impact.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        let mut final_score = (baseline + accumulated_impact).clamp(0.0, 1.0);
    
        // Apply cortisol-specific circadian rhythm modulation
        if primitive == Primitive::Cortisol {
            let circadian_multiplier = Self::cortisol_circadian_multiplier(estimation_time);
            let awakening_boost = Self::cortisol_awakening_boost(events, estimation_time);
            
            // Circadian rhythm sets the healthy baseline for this time of day
            // Even a completely stress-free person has cortisol following this rhythm
            let healthy_baseline = 0.15 + (0.50 * circadian_multiplier);  // Range: 0.15 (night nadir) to 0.65 (morning peak)
            
            // Events push cortisol up (stress, caffeine, poor sleep) or slightly down (relaxation, good sleep)
            // Separate positive (stress) and negative (relaxation) contributions
            let stress_load = accumulated_impact.max(0.0);
            let relaxation_effect = accumulated_impact.min(0.0).max(-0.15);  // Cap relaxation reduction at -0.15
            
            // Stress response is amplified by:
            // 1. Awakening boost (CAR effect)
            // 2. Time of day (easier to spike cortisol during natural peak times)
            let circadian_stress_sensitivity = 0.5 + (0.5 * circadian_multiplier);  // 0.5-1.0 range
            let stress_response = stress_load * awakening_boost * circadian_stress_sensitivity;
            
            // Final cortisol is the healthy circadian baseline plus event-driven modulation
            // Stress adds on top, relaxation can reduce slightly but not eliminate the natural rhythm
            final_score = (healthy_baseline + stress_response + relaxation_effect).clamp(0.15, 1.0);
        }
        
        (final_score, contributors)
    }

    fn compute_event_impacts(&self, event: &Event, estimation_time: DateTime<Utc>) -> HashMap<String, f64> {
        match event.event_type.as_str() {
            "sleep" => compute_sleep_impacts(event),
            "light_exposure" => compute_light_impacts(event, estimation_time),
            "meal" => compute_meal_impacts(event),
            "caffeine" => compute_caffeine_impacts(event),
            "exercise" => compute_exercise_impacts(event),
            "nap" => compute_nap_impacts(event),
            "stress_event" => compute_stress_impacts(event),
            "social_interaction" => compute_social_impacts(event),
            "screen_time" => compute_screen_impacts(event),
            "interruption" => compute_interruption_impacts(event),
            "wake" => HashMap::new(),
            // Health measurements don't directly impact primitives via this path
            _ if event.event_type.starts_with("health_") => HashMap::new(),
            _ => HashMap::new(),
        }
    }

    fn compute_caffeine_dopamine_boost(&self, events: &[Event], estimation_time: DateTime<Utc>) -> f64 {
        let cutoff_time = estimation_time - Duration::hours(12);
        let caffeine_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == "caffeine" && e.timestamp >= cutoff_time && e.timestamp <= estimation_time)
            .collect();

        let mut total_boost = 0.0;
        for event in caffeine_events {
            let dose_mg = event.properties.get("dose_mg")
                .and_then(|v| v.as_f64())
                .unwrap_or(100.0);
            
            let hours_ago = (estimation_time - event.timestamp).num_minutes() as f64 / 60.0;
            let plasma_concentration = dose_mg * (-0.15 * hours_ago).exp();
            let boost = (plasma_concentration / 100.0).min(0.25);
            total_boost += boost;
        }

        total_boost.min(0.4)
    }

    fn compute_adenosine_special(&self, events: &[Event], estimation_time: DateTime<Utc>) -> (f64, Vec<EventContribution>) {
        let mut contributors = Vec::new();
        
        // Find the most recent wake time at or before estimation_time
        let mut most_recent_wake: Option<DateTime<Utc>> = None;
        for event in events.iter() {
            if event.timestamp > estimation_time {
                continue;
            }
            if event.event_type == "wake" {
                // A wake event marks the start of a wake interval
                most_recent_wake = Some(event.timestamp);
            } else if event.event_type == "sleep" {
                // Use sleep end time if it exists and is before estimation_time
                if let Some(end_time) = event.end_timestamp {
                    if end_time <= estimation_time {
                        most_recent_wake = Some(end_time);
                    }
                }
            }
        }
        
        let hours_awake = if let Some(wake_time) = most_recent_wake {
            ((estimation_time - wake_time).num_minutes() as f64 / 60.0).max(0.0)
        } else {
            8.0
        };
        
        // Exponential accumulation toward 1.0 with ~16h time constant
        let base_accumulation = (1.0 - (-hours_awake / 16.0).exp()).clamp(0.0, 1.0);
        
        // Apply sleep clearance using the end of sleep as the effective time
        let cutoff_time = estimation_time - Duration::hours(20);
        let sleep_events: Vec<_> = events
            .iter()
            .filter(|e| {
                if e.event_type != "sleep" {
                    return false;
                }
                if let Some(end_ts) = e.end_timestamp {
                    end_ts >= cutoff_time && end_ts <= estimation_time
                } else {
                    false
                }
            })
            .collect();
        
        let mut sleep_clearance = 0.0;
        for event in sleep_events {
            let impacts = compute_sleep_impacts(event);
            if let Some(&clearance) = impacts.get("adenosine") {
                // Compute decay from the end of sleep (when clearance manifests)
                let end_ts = event.end_timestamp.unwrap_or(event.timestamp);
                let hours_ago = (estimation_time - end_ts).num_minutes() as f64 / 60.0;
                let decay = Self::exponential_decay(hours_ago, 16.0);
                sleep_clearance += clearance * decay;
                
                contributors.push(EventContribution {
                    event_id: event.event_id.clone(),
                    event_type: "sleep".to_string(),
                    impact: clearance,
                    decayed_impact: clearance * decay,
                    hours_ago,
                });
            }
        }
        
        let cutoff_caffeine = estimation_time - Duration::hours(12);
        let caffeine_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == "caffeine" && e.timestamp >= cutoff_caffeine && e.timestamp <= estimation_time)
            .collect();
        
        let mut caffeine_suppression = 0.0;
        for event in caffeine_events {
            let impacts = compute_caffeine_impacts(event);
            if let Some(&suppression) = impacts.get("adenosine") {
                let hours_ago = (estimation_time - event.timestamp).num_minutes() as f64 / 60.0;
                let decay = Self::exponential_decay(hours_ago, 5.0);
                caffeine_suppression += suppression * decay;
                
                contributors.push(EventContribution {
                    event_id: event.event_id.clone(),
                    event_type: "caffeine".to_string(),
                    impact: suppression,
                    decayed_impact: suppression * decay,
                    hours_ago,
                });
            }
        }
        
        let nap_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == "nap" && e.timestamp >= cutoff_time && e.timestamp <= estimation_time)
            .collect();
        
        for event in nap_events {
            let impacts = compute_nap_impacts(event);
            if let Some(&clearance) = impacts.get("adenosine") {
                let hours_ago = (estimation_time - event.timestamp).num_minutes() as f64 / 60.0;
                let decay = Self::exponential_decay(hours_ago, 8.0);
                sleep_clearance += clearance * decay;
                
                contributors.push(EventContribution {
                    event_id: event.event_id.clone(),
                    event_type: "nap".to_string(),
                    impact: clearance,
                    decayed_impact: clearance * decay,
                    hours_ago,
                });
            }
        }
        
        let baseline = self.baseline_values.get("adenosine").copied().unwrap_or(0.3);
        let final_score = (baseline + base_accumulation + sleep_clearance + caffeine_suppression).clamp(0.0, 1.0);
        
        contributors.push(EventContribution {
            event_id: "accumulated_wake_time".to_string(),
            event_type: "wake_accumulation".to_string(),
            impact: base_accumulation,
            decayed_impact: base_accumulation,
            hours_ago: 0.0,
        });
        
        contributors.sort_by(|a, b| {
            b.decayed_impact
                .abs()
                .partial_cmp(&a.decayed_impact.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        (final_score, contributors)
    }

    fn compute_circadian_phase_special(&self, events: &[Event], estimation_time: DateTime<Utc>) -> (f64, Vec<EventContribution>) {
        let mut contributors = Vec::new();
        
        let _hour_of_day = estimation_time.hour() as f64 + (estimation_time.minute() as f64 / 60.0);
        
        let ideal_sleep_time = 23.0;
        let _ideal_wake_time = 7.0;
        
        let sleep_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == "sleep")
            .collect();
        
        let mut avg_sleep_hour = ideal_sleep_time;
        let mut sleep_count = 0;
        
        for event in sleep_events.iter().take(7) {
            let sleep_hour = event.timestamp.hour() as f64;
            avg_sleep_hour += sleep_hour;
            sleep_count += 1;
        }
        
        if sleep_count > 0 {
            avg_sleep_hour /= sleep_count as f64 + 1.0;
        }
        
        let phase_offset = (avg_sleep_hour - ideal_sleep_time) / 10.0;
        
        let cutoff_time = estimation_time - Duration::hours(168);
        let light_events: Vec<_> = events
            .iter()
            .filter(|e| e.event_type == "light_exposure" && e.timestamp >= cutoff_time && e.timestamp <= estimation_time)
            .collect();
        
        let mut light_adjustment = 0.0;
        for event in light_events {
            let impacts = compute_light_impacts(event, estimation_time);
            if let Some(&impact) = impacts.get("circadian_phase") {
                let hours_ago = (estimation_time - event.timestamp).num_minutes() as f64 / 60.0;
                let decay = Self::exponential_decay(hours_ago, 72.0);
                light_adjustment += impact * decay;
                
                contributors.push(EventContribution {
                    event_id: event.event_id.clone(),
                    event_type: "light_exposure".to_string(),
                    impact,
                    decayed_impact: impact * decay,
                    hours_ago,
                });
            }
        }
        
        let baseline = 0.5;
        let final_score = (baseline + phase_offset + light_adjustment).clamp(0.0, 1.0);
        
        (final_score, contributors)
    }

    fn compute_monoamine_scores(
        &self,
        primitive: Primitive,
        events: &[Event],
        estimation_time: DateTime<Utc>,
    ) -> (f64, f64, f64, Vec<EventContribution>) {
        let acute_config = ContextConfig::acute_for_monoamine(primitive);
        let chronic_config = ContextConfig::chronic_for_monoamine(primitive);
        
        let baseline = self.baseline_values
            .get(primitive.as_str())
            .copied()
            .unwrap_or(0.5);

        let acute_cutoff = estimation_time - Duration::hours(acute_config.window_hours);
        let chronic_cutoff = estimation_time - Duration::hours(chronic_config.window_hours);
        
        let _acute_events: Vec<_> = events
            .iter()
            .filter(|e| e.timestamp >= acute_cutoff && e.timestamp <= estimation_time)
            .collect();
        
        let chronic_events: Vec<_> = events
            .iter()
            .filter(|e| e.timestamp >= chronic_cutoff && e.timestamp <= estimation_time)
            .collect();

        let mut acute_impact = 0.0;
        let mut chronic_impact = 0.0;
        let mut all_contributors = Vec::new();

        for event in chronic_events {
            let hours_ago = (estimation_time - event.timestamp).num_minutes() as f64 / 60.0;
            let impacts = self.compute_event_impacts(event, estimation_time);
            
            if let Some(&raw_impact) = impacts.get(primitive.as_str()) {
                let chronic_decay = Self::exponential_decay(hours_ago, chronic_config.decay_half_life_hours);
                let chronic_contribution = raw_impact * chronic_decay;
                chronic_impact += chronic_contribution;
                
                if hours_ago <= acute_config.window_hours as f64 {
                    let acute_decay = Self::exponential_decay(hours_ago, acute_config.decay_half_life_hours);
                    let acute_contribution = raw_impact * acute_decay;
                    acute_impact += acute_contribution;
                }
                
                all_contributors.push(EventContribution {
                    event_id: event.event_id.clone(),
                    event_type: event.event_type.clone(),
                    impact: raw_impact,
                    decayed_impact: chronic_contribution,
                    hours_ago,
                });
            }
        }

        all_contributors.sort_by(|a, b| {
            b.decayed_impact
                .abs()
                .partial_cmp(&a.decayed_impact.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let acute_score = (baseline + acute_impact).clamp(0.0, 1.0);
        let chronic_score = (baseline + chronic_impact).clamp(0.0, 1.0);
        let combined_score = (chronic_score * 0.7 + acute_score * 0.3).clamp(0.0, 1.0);

        (acute_score, chronic_score, combined_score, all_contributors)
    }

    fn exponential_decay(hours_ago: f64, half_life_hours: f64) -> f64 {
        let lambda = 0.693147 / half_life_hours;
        (-lambda * hours_ago).exp()
    }

    /// Calculate cortisol circadian multiplier based on time of day
    /// Returns a value between 0.2 (nadir at midnight) and 1.0 (peak in morning)
    fn cortisol_circadian_multiplier(estimation_time: DateTime<Utc>) -> f64 {
        let hour_of_day = estimation_time.hour() as f64 + (estimation_time.minute() as f64 / 60.0);
        
        // Cortisol follows a well-established circadian rhythm:
        // - Nadir around midnight (0.2-0.3)
        // - Rises during late sleep starting around 2-3 AM
        // - Peaks around 8:30 AM (1.0)
        // - Gradually declines throughout the day
        // - Returns to nadir by midnight
        
        match hour_of_day {
            h if h >= 0.0 && h < 2.0 => {
                // Midnight to 2 AM: at nadir
                0.25
            }
            h if h >= 2.0 && h < 6.0 => {
                // 2 AM to 6 AM: pre-awakening rise (0.25 -> 0.6)
                let progress = (h - 2.0) / 4.0;
                0.25 + (0.35 * progress)
            }
            h if h >= 6.0 && h < 9.0 => {
                // 6 AM to 9 AM: morning peak including CAR (0.6 -> 1.0 -> 0.9)
                let progress = (h - 6.0) / 3.0;
                if progress < 0.5 {
                    // Rising to peak
                    0.6 + (0.4 * progress * 2.0)
                } else {
                    // Just past peak, slight decline
                    1.0 - (0.1 * (progress - 0.5) * 2.0)
                }
            }
            h if h >= 9.0 && h < 12.0 => {
                // 9 AM to noon: post-peak decline (0.9 -> 0.7)
                let progress = (h - 9.0) / 3.0;
                0.9 - (0.2 * progress)
            }
            h if h >= 12.0 && h < 18.0 => {
                // Noon to 6 PM: afternoon decline (0.7 -> 0.45)
                let progress = (h - 12.0) / 6.0;
                0.7 - (0.25 * progress)
            }
            h if h >= 18.0 && h < 22.0 => {
                // 6 PM to 10 PM: evening decline (0.45 -> 0.3)
                let progress = (h - 18.0) / 4.0;
                0.45 - (0.15 * progress)
            }
            _ => {
                // 10 PM to midnight: approaching nadir (0.3 -> 0.25)
                let progress = (hour_of_day - 22.0) / 2.0;
                0.3 - (0.05 * progress)
            }
        }
    }

    /// Calculate cortisol awakening response (CAR) boost
    /// Returns a multiplier (1.0 baseline, up to 1.75 for recent wake)
    fn cortisol_awakening_boost(events: &[Event], estimation_time: DateTime<Utc>) -> f64 {
        // Find the most recent wake event within the last 2 hours
        let lookback_time = estimation_time - Duration::hours(2);
        
        let mut most_recent_wake: Option<DateTime<Utc>> = None;
        for event in events.iter().rev() {
            if event.timestamp < lookback_time {
                break;
            }
            if event.timestamp > estimation_time {
                continue;
            }
            if event.event_type == "wake" {
                most_recent_wake = Some(event.timestamp);
                break;
            }
        }
        
        if let Some(wake_time) = most_recent_wake {
            let minutes_since_wake = (estimation_time - wake_time).num_minutes() as f64;
            
            // Cortisol Awakening Response (CAR):
            // - Peaks at 30-45 minutes after waking
            // - Can increase cortisol by 50-75% (we'll use 1.75x multiplier at peak)
            // - Returns to baseline by 90-120 minutes
            
            if minutes_since_wake <= 60.0 {
                // First hour: rise to peak at 30-40 minutes
                let peak_time = 35.0; // minutes
                if minutes_since_wake <= peak_time {
                    // Rising to peak
                    1.0 + (0.75 * (minutes_since_wake / peak_time))
                } else {
                    // Declining from peak
                    let decline_progress = (minutes_since_wake - peak_time) / (60.0 - peak_time);
                    1.75 - (0.35 * decline_progress)
                }
            } else if minutes_since_wake <= 120.0 {
                // Second hour: gradual return to baseline
                let progress = (minutes_since_wake - 60.0) / 60.0;
                1.4 - (0.4 * progress)
            } else {
                // After 2 hours, CAR effect is gone
                1.0
            }
        } else {
            // No recent wake event, no CAR boost
            1.0
        }
    }

    fn compute_sleep_drive(&self, adenosine: f64, circadian_phase: f64, estimation_time: DateTime<Utc>) -> f64 {
        let hour_of_day = estimation_time.hour() as f64 + (estimation_time.minute() as f64 / 60.0);
        let circadian_sleep_pressure = Self::circadian_sleep_pressure(hour_of_day, circadian_phase);
        
        let homeostatic_component = adenosine;
        let circadian_component = circadian_sleep_pressure;
        
        let sleep_drive = (homeostatic_component * 0.6 + circadian_component * 0.4).clamp(0.0, 1.0);
        sleep_drive
    }

    fn circadian_sleep_pressure(hour_of_day: f64, phase: f64) -> f64 {
        let phase_offset = (phase - 0.5) * 4.0;
        let adjusted_hour = (hour_of_day + phase_offset + 24.0) % 24.0;
        
        let peak_sleep_hour = 3.0;
        let peak_wake_hour = 15.0;
        
        let distance_from_sleep_peak = (adjusted_hour - peak_sleep_hour).abs();
        let distance_from_wake_peak = (adjusted_hour - peak_wake_hour).abs();
        
        let sleep_pressure = if distance_from_sleep_peak < 6.0 {
            0.7 + 0.3 * (1.0 - distance_from_sleep_peak / 6.0)
        } else if distance_from_wake_peak < 3.0 {
            0.2
        } else {
            0.5
        };
        
        sleep_pressure.clamp(0.0, 1.0)
    }

    fn detect_sequences(&self, events: &[Event], estimation_time: DateTime<Utc>) -> Vec<DetectedSequence> {
        let mut sequences = Vec::new();
        
        let lookback_hours = 72;
        let cutoff_time = estimation_time - Duration::hours(lookback_hours);
        let recent_events: Vec<_> = events
            .iter()
            .filter(|e| e.timestamp >= cutoff_time && e.timestamp <= estimation_time)
            .collect();

        let recent_sleep_events: Vec<_> = recent_events
            .iter()
            .filter(|e| e.event_type == "sleep")
            .collect();
        
        if recent_sleep_events.len() >= 2 {
            let mut poor_sleep_count = 0;
            for event in recent_sleep_events.iter().take(3) {
                let quality = event.properties.get("quality")
                    .and_then(|v| v.as_str())
                    .unwrap_or("good");
                if quality == "poor" || quality == "fair" {
                    poor_sleep_count += 1;
                }
            }
            
            if poor_sleep_count >= 2 {
                sequences.push(DetectedSequence {
                    pattern_name: "chronic_sleep_deprivation".to_string(),
                    events: recent_sleep_events.iter().take(3).map(|e| e.event_id.clone()).collect(),
                    impact_on_primitive: "dopamine".to_string(),
                    adjustment: -0.2,
                });
                
                sequences.push(DetectedSequence {
                    pattern_name: "chronic_sleep_deprivation".to_string(),
                    events: recent_sleep_events.iter().take(3).map(|e| e.event_id.clone()).collect(),
                    impact_on_primitive: "serotonin".to_string(),
                    adjustment: -0.15,
                });
            }
        }

        sequences
    }

    fn apply_cross_primitive_modifiers(&self, scores: &HashMap<String, f64>) -> (HashMap<String, f64>, HashMap<String, f64>) {
        let mut modified_scores = scores.clone();
        let mut effective_monoamines = HashMap::new();
        
        let adenosine = scores.get("adenosine").copied().unwrap_or(0.5);
        let cortisol = scores.get("cortisol").copied().unwrap_or(0.5);
        
        if let Some(dopamine) = scores.get("dopamine") {
            let adenosine_suppression = if adenosine > 0.6 {
                -0.15 * (adenosine - 0.6)
            } else {
                0.0
            };
            
            let cortisol_suppression = if cortisol > 0.6 {
                -0.2 * (cortisol - 0.6)
            } else {
                0.0
            };
            
            let modified_dopamine = (dopamine + adenosine_suppression + cortisol_suppression).clamp(0.0, 1.0);
            modified_scores.insert("dopamine".to_string(), modified_dopamine);
        }
        
        if let Some(serotonin) = scores.get("serotonin") {
            let cortisol_suppression = if cortisol > 0.6 {
                -0.15 * (cortisol - 0.6)
            } else {
                0.0
            };
            
            let modified_serotonin = (serotonin + cortisol_suppression).clamp(0.0, 1.0);
            modified_scores.insert("serotonin".to_string(), modified_serotonin);
        }
        
        let dopamine = modified_scores.get("dopamine").copied().unwrap_or(0.5);
        let serotonin = modified_scores.get("serotonin").copied().unwrap_or(0.5);
        
        let inhibition_strength = 0.15;
        let da_inhibits_ser = dopamine * inhibition_strength;
        let ser_inhibits_da = serotonin * inhibition_strength;
        
        let effective_dopamine = (dopamine - ser_inhibits_da).max(0.0);
        let effective_serotonin = (serotonin - da_inhibits_ser).max(0.0);
        
        effective_monoamines.insert("dopamine".to_string(), effective_dopamine);
        effective_monoamines.insert("serotonin".to_string(), effective_serotonin);
        
        (modified_scores, effective_monoamines)
    }

    fn compute_functional_state(dopamine: f64, serotonin: f64) -> FunctionalState {
        let _ratio = if serotonin > 0.01 {
            dopamine / serotonin
        } else {
            dopamine / 0.01
        };
        
        if dopamine >= 0.6 && serotonin >= 0.6 {
            FunctionalState {
                state_type: "Optimal".to_string(),
                description: "Both motivation and mood are strong. Ideal state for productivity and well-being.".to_string(),
                recommendations: vec![
                    "Maintain current patterns".to_string(),
                    "This is a good time for challenging work or important decisions".to_string(),
                ],
            }
        } else if dopamine >= 0.6 && serotonin < 0.5 {
            FunctionalState {
                state_type: "Driven but Anxious".to_string(),
                description: "High motivation but low contentment. Risk of stress and burnout.".to_string(),
                recommendations: vec![
                    "Practice stress-reduction techniques".to_string(),
                    "Increase serotonin: social connection, outdoor time, balanced meals".to_string(),
                    "Avoid overcommitting to new projects".to_string(),
                ],
            }
        } else if dopamine < 0.5 && serotonin >= 0.6 {
            FunctionalState {
                state_type: "Content but Unmotivated".to_string(),
                description: "Good mood but low drive. May struggle with initiation and focus.".to_string(),
                recommendations: vec![
                    "Boost dopamine: exercise (especially HIIT), achievement tasks, protein-rich meals".to_string(),
                    "Set small, concrete goals to build momentum".to_string(),
                    "Consider caffeine in moderation (morning only)".to_string(),
                ],
            }
        } else {
            FunctionalState {
                state_type: "Depleted".to_string(),
                description: "Both motivation and mood are low. Recovery is the priority.".to_string(),
                recommendations: vec![
                    "Prioritize rest and sleep".to_string(),
                    "Avoid demanding decisions or high-stress situations".to_string(),
                    "Gentle exercise, social connection, and balanced nutrition".to_string(),
                ],
            }
        }
    }
}

// ============================================================================
// MAIN AND PRINTING
// ============================================================================