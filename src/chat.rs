// chat.rs - Enhanced chat bridge with full neurological context
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{PrimitiveEstimator, EstimationResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    // Optional: specify which profile or provide custom event data
    pub user_id: Option<String>,
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

#[derive(Serialize)]
struct OpenAIChatBody {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    stream: bool,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: ChatMessage,
}

/// Generate a comprehensive context string from estimation results
fn format_neurological_context(result: &EstimationResult, recent_events: &[crate::Event]) -> String {
    let mut context = String::new();
    
    // 1. Current functional state
    context.push_str(&format!(
        "CURRENT STATE: {}\n{}\n\n",
        result.functional_state.state_type,
        result.functional_state.description
    ));
    
    // 2. Key metrics
    context.push_str("KEY METRICS:\n");
    context.push_str(&format!("- Sleep Drive: {:.1}% (0=alert, 100=exhausted)\n", result.sleep_drive * 100.0));
    context.push_str(&format!("- Dopamine/Serotonin Ratio: {:.2}\n", result.dopamine_serotonin_ratio));
    context.push_str("\n");
    
    // 3. Neurological primitives with interpretations
    context.push_str("NEUROLOGICAL PRIMITIVES (0.0-1.0 scale):\n");
    
    for (name, state) in &result.primitives {
        let score = state.modified_score;
        let interpretation = interpret_primitive_score(name, score);
        
        context.push_str(&format!(
            "- {}: {:.2} - {}\n",
            name.to_uppercase(),
            score,
            interpretation
        ));
        
        // Show top contributing events
        if !state.contributors.is_empty() {
            let top_contributors: Vec<_> = state.contributors
                .iter()
                .take(3)
                .collect();
            
            if !top_contributors.is_empty() {
                context.push_str("  Recent influences: ");
                for (i, contrib) in top_contributors.iter().enumerate() {
                    if i > 0 { context.push_str(", "); }
                    context.push_str(&format!(
                        "{} {:.1}h ago ({:+.2})",
                        contrib.event_type,
                        contrib.hours_ago,
                        contrib.decayed_impact
                    ));
                }
                context.push_str("\n");
            }
        }
        
        // For monoamines, show acute vs chronic if available
        if let (Some(acute), Some(chronic)) = (state.acute_score, state.chronic_score) {
            context.push_str(&format!(
                "  Acute (recent): {:.2}, Chronic (baseline): {:.2}\n",
                acute, chronic
            ));
        }
    }
    context.push_str("\n");
    
    // 4. Detected patterns
    if !result.detected_sequences.is_empty() {
        context.push_str("DETECTED PATTERNS:\n");
        for seq in &result.detected_sequences {
            context.push_str(&format!(
                "- {}: affects {} ({:+.2})\n",
                seq.pattern_name,
                seq.impact_on_primitive,
                seq.adjustment
            ));
        }
        context.push_str("\n");
    }
    
    // 5. Recent significant events (last 12 hours)
    let now = chrono::Utc::now();
    let recent_cutoff = now - chrono::Duration::hours(12);
    let recent: Vec<_> = recent_events
        .iter()
        .filter(|e| e.timestamp >= recent_cutoff)
        .collect();
    
    if !recent.is_empty() {
        context.push_str("RECENT EVENTS (last 12 hours):\n");
        for event in recent.iter().rev().take(10) {
            let hours_ago = (now - event.timestamp).num_minutes() as f64 / 60.0;
            context.push_str(&format!(
                "- {} {:.1}h ago",
                event.event_type,
                hours_ago
            ));
            
            // Add relevant properties
            match event.event_type.as_str() {
                "sleep" => {
                    if let Some(duration) = event.properties.get("duration_hours") {
                        if let Some(quality) = event.properties.get("quality") {
                            context.push_str(&format!(": {} hours, {} quality", duration, quality));
                        }
                    }
                }
                "caffeine" => {
                    if let Some(dose) = event.properties.get("dose_mg") {
                        context.push_str(&format!(": {}mg", dose));
                    }
                }
                "exercise" => {
                    if let Some(intensity) = event.properties.get("intensity") {
                        if let Some(duration) = event.properties.get("duration_minutes") {
                            context.push_str(&format!(": {} intensity, {} min", intensity, duration));
                        }
                    }
                }
                "meal" => {
                    if let Some(protein) = event.properties.get("protein_grams") {
                        if let Some(carbs) = event.properties.get("carb_grams") {
                            context.push_str(&format!(": {}g protein, {}g carbs", protein, carbs));
                        }
                    }
                }
                _ => {}
            }
            context.push_str("\n");
        }
        context.push_str("\n");
    }
    
    // 6. Recommendations from functional state
    if !result.functional_state.recommendations.is_empty() {
        context.push_str("RECOMMENDATIONS:\n");
        for rec in &result.functional_state.recommendations {
            context.push_str(&format!("- {}\n", rec));
        }
    }
    
    context
}

/// Provide human-readable interpretation of primitive scores
fn interpret_primitive_score(primitive: &str, score: f64) -> &'static str {
    match primitive {
        "dopamine" => {
            if score > 0.7 { "High motivation and drive" }
            else if score > 0.5 { "Moderate motivation" }
            else if score > 0.3 { "Low motivation" }
            else { "Very low drive, depleted" }
        }
        "serotonin" => {
            if score > 0.7 { "Good mood and contentment" }
            else if score > 0.5 { "Stable mood" }
            else if score > 0.3 { "Low mood, some irritability" }
            else { "Very low mood, high irritability" }
        }
        "norepinephrine" => {
            if score > 0.7 { "High alertness and focus" }
            else if score > 0.5 { "Normal alertness" }
            else if score > 0.3 { "Reduced alertness" }
            else { "Very low alertness, foggy" }
        }
        "adenosine" => {
            if score > 0.7 { "High sleep pressure, very tired" }
            else if score > 0.5 { "Building sleep pressure" }
            else if score > 0.3 { "Low sleep pressure" }
            else { "Recently woken, alert" }
        }
        "cortisol" => {
            if score > 0.7 { "High stress response" }
            else if score > 0.5 { "Moderate stress" }
            else if score > 0.3 { "Normal stress levels" }
            else { "Low stress, calm" }
        }
        "glucose" => {
            if score > 0.7 { "High blood sugar" }
            else if score > 0.5 { "Stable blood sugar" }
            else if score > 0.3 { "Declining blood sugar" }
            else { "Low blood sugar, need fuel" }
        }
        "circadian_phase" => {
            if score > 0.7 { "Aligned with natural rhythm" }
            else if score > 0.5 { "Moderate circadian alignment" }
            else if score > 0.3 { "Some circadian disruption" }
            else { "Significant circadian misalignment" }
        }
        _ => "Unknown primitive"
    }
}

/// Perform chat completion with full neurological context
pub async fn chat_reply(req: ChatRequest) -> Result<ChatResponse, String> {
    // Get event data based on profile_id or default to "healthy"
    let profile_id = req.profile_id.as_deref().unwrap_or("healthy");
    let event_data = crate::generate_profile_events(profile_id, 7);
    
    // Run the estimator to get current state
    let estimator = PrimitiveEstimator::new();
    let estimation = estimator.estimate_at_time(
        &event_data.events,
        chrono::Utc::now(),
    );
    
    // Generate rich neurological context
    let neuro_context = format_neurological_context(&estimation, &event_data.events);
    
    // Create system message with context
    let system_message = format!(
        "You are an AI assistant with deep access to the user's neurobiological state. \
You can see their current levels of neurotransmitters, hormones, and metabolic markers, \
as well as the recent events (sleep, meals, exercise, caffeine, stress) that influenced them.\n\n\
Use this information to provide insightful, personalized answers about how they're feeling \
and why, what to expect, and what they can do to change their state.\n\n\
When answering:\n\
- Reference specific primitives and events when relevant\n\
- Explain causal relationships (e.g., \"Your high adenosine from 6 hours since wake plus \
low glucose from skipping breakfast is why you feel tired\")\n\
- Be specific about timing (\"2 hours ago\", \"this morning\")\n\
- Give actionable advice based on what would help their specific state\n\
- Consider interactions between primitives (e.g., high cortisol suppressing dopamine)\n\n\
CURRENT NEUROLOGICAL STATE:\n\
{}\n\n\
Current time: {}",
        neuro_context,
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
    );
    
    // Build messages array with system context
    let mut messages = Vec::with_capacity(req.messages.len() + 1);
    messages.push(ChatMessage {
        role: "system".to_string(),
        content: system_message,
    });
    messages.extend(req.messages);
    
    // Make API call (rest of the code remains the same)
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in environment or .env file")
        .trim()  // Remove any whitespace
        .to_string();
    
    // Debug: Show first and last 10 chars of API key
    let key_len = api_key.len();
    let debug_key = if key_len > 20 {
        format!("{}...{}", &api_key[..10], &api_key[key_len-10..])
    } else {
        "[too short]".to_string()
    };
    eprintln!("🔑 Using API key: {} (length: {})", debug_key, key_len);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|e| format!("http client error: {}", e))?;

    let chat_body = OpenAIChatBody {
        model: "gpt-4o".to_string(),
        messages,
        temperature: 0.7,
        stream: false,
    };

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&chat_body)
        .send()
        .await
        .map_err(|e| format!("request error: {}", e))?;

    if resp.status().is_success() {
        let parsed: OpenAIChatResponse = resp
            .json()
            .await
            .map_err(|e| format!("decode error: {}", e))?;
        let reply = parsed
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "No reply.".to_string());
        return Ok(ChatResponse { reply });
    }

    // Fallback to Responses API (same as before)
    let status = resp.status();
    let body_text = resp.text().await.unwrap_or_default();

    let responses_input: Vec<Value> = chat_body
        .messages
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": [
                    { "type": "text", "text": m.content }
                ]
            })
        })
        .collect();

    let responses_payload = serde_json::json!({
        "model": "gpt-4o",
        "input": responses_input,
        "max_tokens": 512,
        "temperature": 0.7
    });

    let resp2 = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(&api_key)
        .header("OpenAI-Beta", "responses=v1")
        .json(&responses_payload)
        .send()
        .await
        .map_err(|e| format!("fallback request error: {}", e))?;

    if !resp2.status().is_success() {
        let status2 = resp2.status();
        let text2 = resp2.text().await.unwrap_or_default();
        return Err(format!(
            "primary {}: {}; fallback {}: {}",
            status, truncate(&body_text, 400),
            status2, truncate(&text2, 400)
        ));
    }

    let v: Value = resp2.json().await.map_err(|e| format!("fallback decode error: {}", e))?;
    if let Some(reply) = v.get("output_text").and_then(|t| t.as_str()) {
        return Ok(ChatResponse { reply: reply.to_string() });
    }
    let reply = v.get("output")
        .and_then(|o| o.as_array())
        .and_then(|arr| arr.get(0))
        .and_then(|first| first.get("content"))
        .and_then(|c| c.as_array())
        .and_then(|carr| carr.get(0))
        .and_then(|c0| c0.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or("No reply.");
    Ok(ChatResponse { reply: reply.to_string() })
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}…", &s[..max]) }
}