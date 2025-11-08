# 🧠 Neural Primitive Estimator Demo - Quick Start

## Run the Demo in 3 Steps

### 1. Start the Server

```bash
cd "/Users/djbray/neural inference"
cargo run --release --bin server
```

You should see:
```
🧠 Neural Primitive Estimator Server
=====================================
Starting server at http://localhost:8080
Open your browser and navigate to http://localhost:8080
```

### 2. Open Your Browser

Navigate to: **http://localhost:8080**

### 3. Explore!

- Click on any data profile card (Healthy Routine, Sleep Deprived, etc.)
- Choose a time resolution (how often to sample the data)
- Click "Analyze Profile"
- Watch the interactive chart show how neural primitives change over time
- Scroll down to see detailed state information and recommendations

## What You'll See

### Interactive Chart
- **7 colored lines** representing each neural primitive:
  - 🔴 Dopamine (motivation/focus)
  - 🔵 Serotonin (mood/stability)
  - 🟡 Norepinephrine (alertness)
  - 🟢 Adenosine (sleep pressure)
  - 🟣 Cortisol (stress)
  - 🟠 Glucose (energy)
  - ⚪ Circadian Phase (time alignment)

### Current State Summary
- Real-time values for each primitive
- Confidence scores
- Sleep drive indicator

### Functional State & Recommendations
- Overall mental/physical state assessment
- Personalized recommendations
- Physiological validation adjustments (if applicable)

## Data Profiles Explained

### 1. **Healthy Routine** ✅
Perfect sleep, balanced nutrition, regular exercise. See optimal primitive balance.

### 2. **Sleep Deprived** 😴
Multiple nights of 4.5-hour poor sleep, excessive caffeine. Watch dopamine and serotonin decline.

### 3. **High Stress** 😰
Uncontrollable stressors, frequent interruptions. See cortisol spike and monoamines suffer.

### 4. **Athlete Training** 💪
HIIT workouts, high-protein meals, excellent sleep. Observe high dopamine and recovery patterns.

### 5. **Shift Worker** 🌙
Rotating sleep schedule, circadian misalignment. Watch the chaos unfold.

## Tips for Exploration

- **Compare profiles**: Run multiple profiles and note the differences
- **Adjust resolution**: Higher resolution (1-2 hours) shows more detail
- **Watch the patterns**: Notice how events affect primitives hours later (decay)
- **Read recommendations**: The system provides actionable insights

## Stopping the Server

Press `Ctrl+C` in the terminal where the server is running.

## Troubleshooting

**Port 8080 in use?**
Edit `src/bin/server.rs` line 64 to change the port:
```rust
.bind(("127.0.0.1", 8081))?
```

**Chart not loading?**
Check your internet connection (Chart.js loads from CDN) or check browser console for errors.

**Build errors?**
```bash
cargo clean
cargo build --release --bin server
```

## Next Steps

- Read `DEMO_README.md` for detailed technical information
- Explore `doc.md` for the research behind the computations
- Check `src/profiles.rs` to see how profiles are generated
- Try the CLI: `cargo run --bin cli`

---

**Enjoy exploring neural primitives!** 🎉

