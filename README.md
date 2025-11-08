# Neural Inference

A Rust-based system that estimates neurobiological primitives (dopamine, serotonin, norepinephrine, adenosine, cortisol, glucose, circadian phase) from activity data using research-based computational models.

## What It Does

Takes activity data (sleep, exercise, meals, caffeine, stress, etc.) and computes biological state estimates based on neuroscience research. Instead of hardcoded impacts, it derives effects from event properties using peer-reviewed formulas.

## Quick Start

### Run the CLI

```bash
# With your own data file
cargo run --bin cli -- /path/to/your_data.json

# With mock data
cargo run --bin cli -- mock_data.json

# Use a preset profile
cargo run --bin cli -- --profile chronically_sleep_deprived
```

### Run the Web Server

```bash
cargo run --bin server
```

Then open `http://localhost:8080` in your browser.

### Available Profiles

Pre-built profiles for testing:
- `healthy_balanced` - Well-rested, regular exercise, good nutrition
- `chronically_sleep_deprived` - Multiple nights of poor sleep
- `caffeine_dependent` - High caffeine intake, withdrawal effects
- `shift_worker` - Night shifts with circadian disruption
- `high_performer` - Intense exercise and optimized schedule
- `stressed_student` - Poor sleep, high stress, irregular meals

## Data Format

Input is JSON with events that have properties (not hardcoded impacts):

```json
{
  "user_id": "user_123",
  "events": [
    {
      "event_id": "evt_1",
      "event_type": "sleep",
      "timestamp": "2025-01-15T23:00:00Z",
      "end_timestamp": "2025-01-16T07:00:00Z",
      "properties": {
        "duration_hours": 8.0,
        "quality": "good",
        "efficiency": 0.92
      }
    },
    {
      "event_id": "evt_2",
      "event_type": "caffeine",
      "timestamp": "2025-01-16T08:00:00Z",
      "properties": {
        "dose_mg": 100
      }
    },
    {
      "event_id": "evt_3",
      "event_type": "exercise",
      "timestamp": "2025-01-16T17:00:00Z",
      "properties": {
        "type": "cardio",
        "intensity": "moderate",
        "duration_minutes": 30
      }
    }
  ]
}
```

## Event Types

- `sleep` - Duration, quality, efficiency
- `caffeine` - Dose in mg
- `exercise` - Type, intensity, duration
- `meal` - Macros (protein/carbs/fats %), glycemic index
- `light` - Intensity in lux, wavelength
- `stress` - Severity, controllability, social evaluation
- `social` - Type (positive/negative), quality
- `screen` - Duration, content type, blue light
- `nap` - Duration
- `interruption` - Frequency

## Output

The system produces:
- Current state estimates for all 7 primitives
- Contributing events with computed impacts
- Detected patterns (sleep deprivation, withdrawal, synergies)
- Sleep drive analysis (homeostatic + circadian)
- Natural language interpretation

## API Key (Optional)

For natural language interpretation via OpenAI:
```bash
export OPENAI_API_KEY="your-key-here"
```

The system works without an API key; interpretation just won't be available.

## Project Structure

```
src/
├── lib.rs         # Core estimation engine
├── chat.rs        # Natural language interpretation
├── profiles.rs    # Pre-built test profiles
├── bin/
│   ├── cli.rs     # Command-line interface
│   └── server.rs  # Web server demo
static/
├── index.html     # Web interface
└── chat.js        # Frontend JS
```

## Building

```bash
# Development
cargo build

# Optimized release
cargo build --release
./target/release/server
```

## License

MIT
