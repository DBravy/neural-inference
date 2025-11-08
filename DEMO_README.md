# Neural Primitive Estimator - Interactive Web Demo

## Overview

This interactive web demo allows you to explore how neurobiological primitives (dopamine, serotonin, norepinephrine, adenosine, cortisol, glucose, and circadian phase) change over time under different lifestyle scenarios.

## Features

- **5 Pre-configured Profiles**:
  - **Healthy Routine**: Consistent sleep, balanced meals, regular exercise
  - **Sleep Deprived**: Multiple nights of poor sleep with high caffeine use
  - **High Stress**: Work pressure, frequent stress events, irregular eating
  - **Athlete Training**: Intense exercise with optimized nutrition and recovery
  - **Shift Worker**: Irregular sleep schedule with circadian misalignment

- **Interactive Visualization**: See all 7 neural primitives plotted over time
- **Real-time Analysis**: Select a profile and watch how primitives evolve
- **Detailed Insights**: View functional state, sleep drive, and physiological validations
- **Adjustable Resolution**: Choose different time granularities (1-6 hours)

## Running the Demo

### Prerequisites

- Rust (1.70 or later) - [Install Rust](https://www.rust-lang.org/tools/install)
- A modern web browser (Chrome, Firefox, Safari, Edge)

### Steps

1. **Build and run the server**:
   ```bash
   cargo run --release --bin server
   ```

2. **Open your browser**:
   Navigate to [http://localhost:8080](http://localhost:8080)

3. **Explore the demo**:
   - Select a data profile (click on a card)
   - Choose a time resolution
   - Click "Analyze Profile"
   - View the interactive chart and detailed results

## Project Structure

```
neural inference/
├── src/
│   ├── lib.rs              # Core estimation logic
│   ├── profiles.rs         # Pre-configured data profiles
│   ├── main.rs             # Simple redirector
│   └── bin/
│       ├── server.rs       # Web server (actix-web)
│       └── cli.rs          # Command-line interface
├── static/
│   └── index.html          # Web UI with Chart.js
├── mock_data.json          # Sample event data
├── Cargo.toml              # Rust dependencies
└── DEMO_README.md          # This file
```

## How It Works

### Backend (Rust)

The server provides two main API endpoints:

- **GET /api/profiles**: Returns available data profiles
- **POST /api/estimate**: Analyzes a profile over time
  ```json
  {
    "profile_id": "healthy",
    "resolution_hours": 4
  }
  ```

### Frontend (HTML/JS/Chart.js)

The web interface:
1. Fetches available profiles
2. Displays them as clickable cards
3. Sends estimation requests to the backend
4. Visualizes results using Chart.js time-series charts
5. Shows detailed state information and recommendations

### Data Profiles

Each profile generates realistic event sequences over 4 days:

- **Healthy Routine**: 8h sleep, morning light exposure, balanced meals, moderate exercise
- **Sleep Deprived**: 4.5h poor sleep, excessive caffeine (>500mg/day), late screens
- **High Stress**: Uncontrollable stress events, interruptions, elevated physiological markers
- **Athlete**: 8.5h excellent sleep, HIIT training, high-protein nutrition
- **Shift Worker**: Rotating sleep schedule, circadian misalignment, irregular meals

## Technical Details

### Neural Primitives Estimated

1. **Dopamine** - Motivation, focus, reward processing
2. **Serotonin** - Mood stability, contentment
3. **Norepinephrine** - Alertness, arousal
4. **Adenosine** - Sleep pressure (Process S)
5. **Circadian Phase** - Time-of-day regulation (Process C)
6. **Cortisol** - Stress response
7. **Glucose** - Energy availability

### Research-Based Computations

All impact functions are derived from peer-reviewed neuroscience research:

- **Sleep**: Adenosine clearance, dopamine receptor upregulation
- **Caffeine**: Adenosine A2A receptor antagonism (ED50 = 65mg)
- **Exercise**: Intensity-dependent dopamine and cortisol responses
- **Nutrition**: Macronutrient effects on neurotransmitter synthesis
- **Light**: Circadian phase shifts (morning advance, evening delay)
- **Stress**: Controllability determines cortisol vs norepinephrine response

See `doc.md` and `FORMULA_REFERENCE.md` for detailed research citations.

### Physiological Validation

The system incorporates physiological measurements to validate/adjust estimates:

- **HRV**: Autonomic nervous system state
- **Heart Rate**: Sympathetic activation
- **Blood Glucose**: Direct metabolic measurement
- **Respiratory Rate**: Anxiety/stress indicator

## Command-Line Interface

If you prefer the CLI:

```bash
# Run estimation on mock_data.json
cargo run --bin cli

# Build CLI binary
cargo build --release --bin cli
./target/release/cli
```

## Development

### Adding New Profiles

Edit `src/profiles.rs` and add a new generation function:

```rust
fn generate_my_profile(base_time: DateTime<Utc>, days: i64) -> Vec<Event> {
    // Generate events
}
```

Then register it in `get_all_profiles()` and `generate_profile_events()`.

### Modifying the UI

Edit `static/index.html` - it's a single-file application with:
- CSS in `<style>` tags
- JavaScript in `<script>` tags
- Chart.js for visualization

### Adjusting Computation Parameters

Edit `src/lib.rs`:
- Context windows: `ContextConfig::for_primitive()`
- Decay rates: `decay_half_life_hours`
- Impact functions: `compute_*_impacts()`

## Performance

- Server startup: ~instant
- Profile generation: <1ms
- Time-series estimation (4 days, 4h resolution): ~10-50ms
- Response times: Typically 10-100ms

## Browser Compatibility

Tested on:
- Chrome 120+
- Firefox 121+
- Safari 17+
- Edge 120+

Requires JavaScript enabled and support for:
- ES6 syntax
- Fetch API
- Canvas (for Chart.js)

## Troubleshooting

### Port 8080 already in use

Change the port in `src/bin/server.rs`:

```rust
.bind(("127.0.0.1", 8081))?  // Change to 8081 or another port
```

### Chart not displaying

- Check browser console for errors
- Ensure Chart.js CDN is accessible
- Verify API responses in Network tab

### Build errors

- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build --release`

## Future Enhancements

Potential additions:
- Custom profile editor
- Export results to CSV/JSON
- Comparison mode (overlay multiple profiles)
- Intervention simulator (test hypothetical changes)
- Real-time data import from fitness trackers

## License

See main project LICENSE.

## Citation

If you use this demo in research or educational contexts, please cite the underlying research papers listed in `doc.md`.

## Contact

For questions or feedback, please open an issue in the repository.

---

**Enjoy exploring your neural primitives!** 🧠

