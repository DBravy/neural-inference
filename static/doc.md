# Neurobiological Primitive Estimator - Research-Based Architecture

## Overview

This system estimates 7 neurobiological primitives (dopamine, serotonin, norepinephrine, adenosine, cortisol, glucose, circadian phase) by **deriving their values from activity data using peer-reviewed neuroscience research**.

## Research-Based Impact Functions

Each activity type has a dedicated computation function based on quantitative research:

### 1. **Sleep** (`compute_sleep_impacts`)
**Research basis**: Sleep affects all 7 primitives through different mechanisms
- **Adenosine**: Clears with ~90min half-life per cycle; 50% clearance in 90min nap
- **Dopamine**: Good sleep restores D2 receptors; <6h causes 10% downregulation/24h
- **Serotonin**: Proportional to quality × duration
- **Cortisol**: Good sleep reduces; deprivation → +37-45% evening cortisol
- **Glucose**: <6h impairs tolerance 40%, insulin sensitivity -30%
- **Circadian**: Late sleep (past midnight) delays phase

**Key thresholds**:
- Duration <6h: metabolic impairment
- Efficiency <67%: reduced benefits
- Duration ≥7h + quality ≥0.8: optimal restoration

### 2. **Light Exposure** (`compute_light_impacts`)
**Research basis**: Intensity, wavelength, and timing determine effects
- **Circadian Phase**: 
  - Morning (6-11am) + ≥2000 lux: Phase advance (-0.8h)
  - Evening (6-10pm) + ≥2000 lux: Phase delay (+0.6h)
  - Saturation above 2000 lux
- **Serotonin**: Morning bright light → 10,000 lux × 30min (SAD treatment, effect size 4.64)
- **Cortisol**: Morning light → 800 lux = +35%, 5000 lux = +50% (CAR enhancement)

**Key thresholds**:
- 100 lux: Minimum circadian effect
- 2000 lux: Saturation for phase shifts
- 10,000 lux: Therapeutic for SAD

### 3. **Nutrition/Meals** (`compute_meal_impacts`)
**Research basis**: Macros determine neurotransmitter precursor availability
- **Glucose**: High-GI produces 2.4× greater AUC; protein dampens response
- **Serotonin**: High-carb (<10% protein) maximizes via insulin-mediated BCAA removal
  - 1-2h lag time for tryptophan/LNAA ratio increase
- **Dopamine**: High-protein provides tyrosine (15-20g adequate; >50g no additional benefit)
  - High-GI foods activate reward circuitry (+8.2% NAc blood flow at 4h post-meal)
- **Circadian**: Meal timing phases peripheral clocks (5h delay → 5.69h glucose rhythm shift)

**Key relationships**:
- Protein <10% + carbs >40g → Serotonin ↑
- Protein >25% → Blocks serotonin, provides tyrosine for dopamine
- Protein 15-20g → Optimal for neurotransmitter synthesis

### 4. **Caffeine** (`compute_caffeine_impacts`)
**Research basis**: Adenosine antagonism with dose-dependent receptor occupancy
- **Receptor occupancy**:
  - A2A receptors: ED50 = 65mg
  - A1 receptors: ED50 = 450mg
  - 200mg → 60% A2A, 30% A1 occupancy
- **Half-life**: 4-6 hours (modeled as 5h: -0.15 × hours)
- **Adenosine**: Blocks receptors (negative adenosine impact)
- **Dopamine**: Increases D2/D3 receptor availability (not direct release at human doses)
- **Norepinephrine**: Dose-dependent (300mg → significant increases)
- **Cortisol**: 30-50% increase in naive individuals (tolerance develops in 5 days)
- **Circadian**: 200mg 3h before bed → 40min phase delay

**Plasma concentration tracking**:
```rust
plasma_conc = dose_mg × e^(-0.15 × hours_ago)
```

### 5. **Exercise** (`compute_exercise_impacts`)
**Research basis**: Intensity-dependent effects across all systems
- **Dopamine**: 
  - HIIT → 16% D2 receptor increase after 6+ weeks
  - Acute peak 0-30min post-exercise
  - Intensity ≥70% VO2max → maximum effect
- **Serotonin**: Cardio increases free tryptophan 30% via lipolysis
- **Norepinephrine**: Immediate release with intensity
- **Cortisol**: 
  - Activates at 60% VO2max threshold
  - Dramatic spike at 80%+ VO2max
- **Glucose**: Depletion rate ∝ Intensity^2.5 (~80min to depletion at lactate steady state)
- **Adenosine**: High-intensity (>80%) accumulates (229% baseline)
- **Circadian**: Morning exercise → -0.3h phase advance; evening → ~0h

**Key thresholds**:
- 30min minimum for BDNF
- 60% VO2max for cortisol activation
- 70-85% max HR for endocannabinoids ("runner's high")
- 80%+ for maximum dopamine/BDNF but also maximum cortisol

### 6. **Naps** (`compute_nap_impacts`)
**Research basis**: Duration determines clearance, timing affects phase
- **Adenosine clearance**: A_after = A_before × e^(-0.0077 × duration_min)
  - 20min nap: 14% clearance, no inertia
  - 90min nap: 50% clearance, inertia risk if interrupted
- **Circadian timing**:
  - Morning naps: Phase delay (reduces morning light benefit)
  - Afternoon (1-3pm): Safe window, minimal phase effect
  - Evening naps: Phase advance (+30 to +60min)

### 7. **Stress Events** (`compute_stress_impacts`)
**Research basis**: **Controllability is KEY** - determines cortisol vs NE response
- **Cortisol**:
  - Uncontrollable + social-evaluative = 3× multiplier (d=0.92, largest response)
  - Peaks 20-30min, recovery 40-60min
  - Controllable stress without social eval = d=-0.08 (minimal cortisol)
- **Norepinephrine**: Controllable stress favors NE (immediate, seconds)
- **Dopamine**: 
  - Controllable → Mild enhancement (task completion, flow)
  - Uncontrollable → Depletion (chronic stress depletes prefrontal dopamine)
- **Glucose**: Stress triggers release

**Formula**:
```rust
cortisol_multiplier = if !controllable && social_evaluative {
    3.0
} else if !controllable {
    2.0
} else if social_evaluative {
    1.5
} else {
    0.5 // Controllable stress
};
```

### 8. **Social Interaction** (`compute_social_impacts`)
**Research basis**: VTA dopamine activation, oxytocin-serotonin coordination
- **Dopamine**:
  - VTA neurons: 53% activate during reciprocal, 42% unilateral, 7% passive
  - Peaks at 27% of interaction bout duration
  - Habituation with familiarity; novelty boosts initial response
- **Serotonin**: Oxytocin-serotonin coordination in NAc required for social reward
- **Cortisol**: 
  - Negative interactions (rejection) → Increase
  - Positive interactions → Stress buffering (-0.15)

**Quality multiplier**:
- Positive: 1.5×
- Neutral: 1.0×
- Negative: 0.3×

**Interaction type multiplier**:
- Reciprocal: 1.0× (baseline)
- Unilateral: 0.79× (42/53)
- Passive: 0.13× (7/53)

### 9. **Screen Time** (`compute_screen_impacts`)
**Research basis**: Blue light circadian disruption, dopamine via variable reinforcement
- **Circadian Phase**:
  - Blue light melatonin suppression ED50: 13-39 lux (timing-dependent)
  - 2h evening exposure ≈ 1.5h phase delay
  - Threshold: 8 lux (any effect), 30 lux (clinical concern)
- **Dopamine**: Interactive content (social media, gaming) provides variable reinforcement
  - Variable ratio schedule → more dopamine than consistent rewards
  - Platform algorithms batch likes → negative prediction error + burst

**Content type dopamine impact**:
- Social media: 0.2 × duration
- Gaming: 0.25 × duration
- Passive: 0.0
- Work: 0.05

### 10. **Interruptions** (`compute_interruption_impacts`)
**Research basis**: Interruptions multiply stress, 23min recovery time
- **Cortisol**: ~2× versus uninterrupted work
- **Norepinephrine**: Sympathetic activation
- **Glucose**: Working memory reset depletes glucose faster

## Special Primitive Computations

### Adenosine (Process S - Sleep Homeostasis)
**Special handling**: Linear accumulation, not decay-based

**Algorithm**:
1. Find most recent wake event
2. Calculate hours awake
3. Apply accumulation formula: `A(t) = 1 - e^(-t/16h)`
4. Subtract sleep clearance (exponentially decayed from past sleep events)
5. Subtract caffeine blockade (plasma concentration tracking)

**Caffeine blockade**:
```rust
plasma_conc = dose_mg × e^(-0.15 × hours_ago)
a2a_occupancy = plasma_conc / (plasma_conc + 65.0)
blockade = 0.5 × a2a_occupancy
```

### Circadian Phase (Process C - Time-of-Day Regulation)
**Special handling**: Tracks phase offset, not quality

**Score interpretation**:
- 0.5 = Baseline phase (on schedule)
- <0.5 = Phase advanced (earlier)
- >0.5 = Phase delayed (later)
- Range: Typically 0.3-0.7 (±3 hours)

**Algorithm**:
1. Aggregate phase-shifting events over 7 days
2. Apply exponential decay (72h half-life)
3. Each event contributes a signed phase offset:
   - Morning light (6-11am, ≥2000 lux): -0.8h (advance)
   - Evening light (6-10pm, ≥2000 lux): +0.6h (delay)
   - Late screens (<3h before sleep): +0.3-0.6h (delay)
   - Morning exercise: -0.3h (advance)
   - Late sleep (past midnight): +0.2h (delay)
4. Add natural drift if no morning light: +0.4h
5. Apply adenosine gating to light sensitivity:
   - Adenosine >0.7: 50% light sensitivity (sleep-deprived)
   - Adenosine 0.5-0.7: 75% sensitivity
   - Adenosine <0.5: 100% sensitivity

**Adenosine gating mechanism**: High homeostatic sleep pressure attenuates light-induced phase shifts through adenosinergic inhibition of SCN photic pathways.

### Sleep Drive (Two-Process Model)
**Formula**: `Sleep Drive = 0.6 × Adenosine + 0.4 × Circadian_Component`

**Circadian component** (Process C):
```rust
// Sinusoidal function peaking at biological night (3am)
// Adjusted for individual phase offset
adjusted_hour = (hour_of_day - phase_offset_hours + 24.0) % 24.0
raw_pressure = -cos(2π × (adjusted_hour - 3.0) / 24.0)
scaled_pressure = 0.3 + 0.4 × (raw_pressure + 1.0) / 2.0
```

**Interpretation**:
- <0.3: Alert, good wakefulness
- 0.3-0.5: Comfortable waking state
- 0.5-0.7: Increasing sleep pressure
- >0.7: Strong urge to sleep

## Dopamine-Serotonin Balance System

**Research basis**: 5-HT2C receptors on VTA dopamine neurons create reciprocal inhibition. This system models both dual time scales and cross-inhibition effects.

### Dual Time Scale Computation

Both dopamine and serotonin are computed at **two time scales**:

1. **Acute Score** (Recent state - what you feel NOW)
   - Dopamine: 12h window, 6h decay half-life
   - Serotonin: 16h window, 8h decay half-life
   - Captures immediate effects of recent events

2. **Chronic Score** (Baseline state - underlying condition)
   - Dopamine: 72h window, 24h decay half-life
   - Serotonin: 96h window, 36h decay half-life
   - Captures sustained patterns and depletion/buildup

**Combined formula**: `combined_score = 0.7 × acute + 0.3 × chronic`

This weighting means recent events (last 12-16h) matter more than older patterns.

### Reciprocal Inhibition

High serotonin suppresses dopamine more strongly than vice versa:

**Serotonin → Dopamine inhibition:**
```rust
if serotonin > 0.6 {
    inhibition = 1.0 - 0.25 × ((serotonin - 0.6) / 0.4)
    dopamine_effective = dopamine_combined × inhibition
}
```

**Dopamine → Serotonin inhibition (weaker):**
```rust
if dopamine > 0.7 {
    inhibition = 1.0 - 0.15 × ((dopamine - 0.7) / 0.3)
    serotonin_effective = serotonin_combined × inhibition
}
```

### DA/5HT Ratio & Functional States

**Ratio**: `dopamine_effective / serotonin_effective`

**Interpretation**:
- **>1.4**: Dopamine-dominant (high drive, lower mood stability)
- **0.65-1.4**: Balanced (optimal flexibility)
- **<0.65**: Serotonin-dominant (stable mood, lower motivation)

**Functional states** (determined by ratio AND absolute levels):

| State | Conditions | Characteristics |
|-------|------------|-----------------|
| **Peak Performance** | Both ≥0.65 | High motivation + stable mood |
| **Depleted** | Both ≤0.40 | Low motivation + low stability, need recovery |
| **Driven but Anxious** | DA ≥0.65, 5HT ≤0.45 OR ratio >1.4 | High drive, reduced stress resilience, burnout risk |
| **Calm but Unmotivated** | DA ≤0.45, 5HT ≥0.65 OR ratio <0.65 | Stable mood, reduced initiative |
| **Balanced (DA-leaning)** | Ratio 1.1-1.4 | Good motivation with reasonable stability |
| **Balanced (5HT-leaning)** | Ratio 0.75-0.9 | Stable mood with moderate motivation |
| **Well-Balanced** | Ratio 0.9-1.1 | Optimal balance, flexible and resilient |

This system captures why activities that boost both neurotransmitters (like quality sleep, morning exercise) are most beneficial - they optimize the ratio rather than maximizing one at the expense of the other.

## Sequence Detection Patterns

These patterns capture non-additive interactions:

### 1. Chronic Sleep Deprivation
**Detection**: 3+ poor-quality sleep events within 72h
**Effects**:
- Dopamine: -0.25 (D2/D3 downregulation accumulates)
- Serotonin: -0.20 (5-HT system dysregulation)

### 2. Caffeine Withdrawal
**Detection**: 
- ≥7 caffeine events in past week
- Average dose ≥100mg/day
- No caffeine in 24-168h
**Effects** (peak at 48h, exponential decay):
- Dopamine: -0.20 (fatigue, reduced motivation)
- Serotonin: -0.15 (mood disturbances)
- Norepinephrine: -0.20 (reduced alertness)
- Cortisol: +0.15 (withdrawal stress response)

**Withdrawal timeline**:
- Onset: 24h after last caffeine
- Peak: 48h (maximum intensity)
- Duration: Up to 168h (7 days) with exponential decay

### 3. Late Caffeine → Sleep Disruption
**Detection**: Caffeine within 9h of sleep + poor/fair sleep quality
**Effects**:
- Cortisol: +0.10 to +0.20 (from sleep disruption cascade)
- Severity scales with dose and timing proximity

**Research**: Caffeine 6h before bed reduces sleep by 45+ min; 100mg needs 8.8h clearance, 200mg needs 13.2h

### 4. Quality Sleep → Morning Exercise Synergy
**Detection**: Good/excellent sleep (≥7h) → exercise within 6h
**Effects**:
- Dopamine: +0.15 (synergistic boost beyond additive)

## Cross-Primitive Modifiers

Second-pass adjustments based on primitive interactions:

| Condition | Effect | Magnitude | Research Basis |
| Circadian misaligned (<0.35 or >0.65) | Suppresses all neurotransmitters | 0.85× | Phase >1h off impairs function |
| Glucose <0.4 | Impairs all brain function | 0.70× | Hypoglycemia effects |
| Adenosine >0.7 | Specifically impairs dopamine | 0.7-1.0× | A2A-D2 receptor heteromers |
| Cortisol >0.7 | Suppresses dopamine & serotonin | 0.85× | Chronic stress studies |

## Physiological Validation Layer

**Purpose**: Objective physiological measurements act as reality-checks on event-based estimates, adjusting scores when biological signals contradict behavioral predictions.

**Health Event Types**: `health_heart_rate`, `health_hrv`, `health_blood_oxygen`, `health_blood_glucose`, `health_body_temperature`, `health_respiratory_rate`, `health_steps`

### Validation Rules

| Measurement | Constraint | Effect | Research Basis |
| **HRV <30ms** | Cortisol floor = 0.6 | Overrides low cortisol predictions | Direct vagal tone measurement; inversely correlates with HPA axis |
| **HR >80 bpm** (resting) | Norepinephrine floor = 0.5 | Catches missed arousal/stress | β1-adrenergic receptor activation |
| **Glucose <70 mg/dL** | Cortisol floor = 0.6, Override glucose | Hypoglycemia triggers counterregulatory response | Glucocorticoid stress response |
| **SpO2 <92%** (sleep) | Reduces dopamine confidence | Poor sleep quality despite duration | Intermittent hypoxia impairs neurotransmitter restoration |
| **RR >18 bpm** | Cortisol floor = 0.5, Serotonin confidence ↓ | Anxiety/stress indicator | Mediated by locus coeruleus, raphe nuclei |
| **Steps <2000/day** | Dopamine confidence ↓ | Low activity contradicts high dopamine | Motor activity requires dopaminergic drive |

**Key Features**:
- **Recency windows**: Only recent measurements affect acute signals (1-4 hours for HR/HRV/glucose)
- **Constraint types**: Floor (minimum), Ceiling (maximum), Override (blend 70% measured/30% predicted), Confidence penalty
- **Graceful degradation**: Missing physiological data doesn't penalize; system falls back to event-based estimation
- **Confidence scoring**: Each primitive gets confidence score (0-1) indicating certainty
- **Explainability**: Every adjustment includes reasoning linked to physiological mechanism

**Example**: If events predict low cortisol but HRV is 28ms and HR is 84 bpm, the validation layer sets cortisol floor at 0.6 and reduces adenosine confidence (high HR contradicts sleep pressure), catching missed stressors.

## Context Windows and Time Decay

Each primitive has biologically-appropriate windows:

| Primitive | Window | Half-Life | Rationale |
| Glucose | 8h | 2h | Fast metabolism |
| Norepinephrine | 12h | 4h | Caffeine + acute stress |
| Adenosine | 20h | 16h | Wake period + debt |
| Cortisol | 48h | 12h | Acute + chronic stress |
| Dopamine | 72h | 24h | Lifestyle patterns |
| Serotonin | 96h | 36h | Mood stability |
| Circadian Phase | 168h | 72h | Weekly entrainment |

**Exponential decay formula**: `e^(-0.693 × hours_ago / half_life)`

## Event Data Structure

Events contain only **activity metadata**, not impacts:

```json
{
  "event_type": "exercise",
  "timestamp": "2025-01-15T10:00:00Z",
  "properties": {
    "duration_minutes": 45,
    "intensity": "moderate",
    "type": "cardio",
    "heart_rate_avg": 145,
    "vo2max_percentage": 65,
    "hours_since_last_meal": 2.0,
    "time_of_day": "morning"
  }
}
```

**Health measurement events** provide physiological validation:

```json
{
  "event_type": "health_hrv",
  "timestamp": "2025-01-15T06:45:00Z",
  "properties": {
    "value": 72.0,
    "unit": "ms",
    "metric": "rmssd",
    "context": "morning_baseline"
  }
}
```

## Algorithm Flow

```
1. Load events from JSON (activity + health data)
2. Compute Adenosine (Process S):
   - Track wake time, apply accumulation formula
   - Subtract sleep clearance (past events)
   - Subtract caffeine blockade (plasma tracking)
3. Compute Circadian Phase (Process C):
   - Aggregate phase-shifting events (7 days)
   - Apply adenosine gating to light sensitivity
   - Apply exponential decay
   - Convert to phase score (0.3-0.7)
4. Compute Sleep Drive:
   - Combine: 0.6 × adenosine + 0.4 × circadian_sleep_propensity
5. For each remaining primitive:
   - Filter events within context window
   - For each event, call research-based impact function
   - Apply time decay to impacts
   - Aggregate weighted impacts
6. Detect sequence patterns
7. Apply sequence adjustments
8. Apply cross-primitive modifiers
9. Apply physiological validation:
   - Extract physiological measurements from health events
   - Generate constraints based on measurement values
   - Apply constraints (floors, ceilings, overrides)
   - Update confidence scores
10. Clamp scores to valid ranges
11. Generate interpretation with confidence indicators
```

## Key Benefits of Research-Based Approach

1. **Transparency**: Impacts traceable to specific research findings
2. **Maintainability**: Update formulas as new research emerges
3. **Consistency**: Same activity always produces same impact (given same properties)
4. **Extensibility**: Add new activity types by creating new impact functions
5. **Interpretability**: Can explain *why* a primitive changed (specific formula + threshold)
6. **Data efficiency**: Richer metadata enables more accurate computations
7. **Validation**: Physiological measurements provide reality-checks, catching missed context and improving accuracy

## Example Output

```
┌─ Dopamine (Motivation/Focus)
│  Score:          0.385 (Confidence: 85%)
│  Status:         🟠 Low
│  Top Contributors:
│    1. chronic_sleep_deprivation (24.0h ago): -0.250
│    2. sleep_clearance (18.0h ago): -0.156
│    3. exercise (3.5h ago): +0.312
└─────────────────────────────────────────────

┌─ Cortisol (Stress Response)
│  Score:          0.650 (Confidence: 72%) ⚠️
│  Status:         🟠 High
│  Physiological Override: HRV 28ms set floor at 0.6
└─────────────────────────────────────────────

╔═══════════════════════════════════════════════════════════════════╗
║                  SLEEP DRIVE (Two-Process Model)                 ║
╚═══════════════════════════════════════════════════════════════════╝

Overall Sleep Drive: 0.782 (🔴 VERY HIGH - Strong urge to sleep)
├─ Homeostatic Component (Process S): 0.850
│  (Adenosine/sleep pressure from time awake)
├─ Circadian Component (Process C):   0.650
│  (Time-of-day sleep propensity)
└─ Combined Sleep Drive:              0.782

🔬 Physiological Validation Adjustments:
   HeartRateVariability → cortisol: 0.450 → 0.600
   Reason: Very low HRV (28.0ms) indicates high stress/cortisol
```

## Research Citations

All formulas are based on peer-reviewed research. Key papers include:

### Caffeine
- Solinas et al. (2002). "Caffeine Induces Dopamine and Glutamate Release in the NAc." *J Neuroscience*
- Drake et al. (2013). "Caffeine Effects on Sleep Taken 0, 3, or 6 Hours before Going to Bed." *J Clinical Sleep Med*
- Juliano & Griffiths (2004). "A critical review of caffeine withdrawal." *Psychopharmacology*
- Reichert et al. (2022). "Adenosine, caffeine, and sleep–wake regulation." *J Sleep Research*

### Sleep & Circadian
- Borbély's Two-Process Model of Sleep Regulation
- Van Dongen et al. (2003). Sleep debt recovery studies
- Wright et al. (2013). Entrainment by camping studies

### Exercise
- Robertson et al. (2016). D2 receptor upregulation from exercise
- Erickson et al. (2011). Exercise and brain plasticity

### Nutrition
- Fernstrom & Wurtman (1971). Tryptophan, LNAA ratios, and brain serotonin
- Lennerz et al. (2013). High-GI foods and reward circuitry activation

### Stress
- Dickerson & Kemeny (2004). Meta-analysis of psychosocial stress (208 studies)
- Maier & Watkins (2005). Controllability and stress responses

(Still working on full research document with complete citations and quantitative relationships)

## Future Extensions

Potential additions to the research-based approach:

1. **Individual calibration**: Adjust formulas based on user's response patterns
2. **Genetic factors**: Incorporate CYP1A2 polymorphisms (caffeine metabolism), ADORA2A (adenosine sensitivity)
3. **Environmental factors**: Temperature, altitude, humidity effects
4. **Medication interactions**: Pharmacological impacts on primitives
5. **Chronic conditions**: Diabetes, depression, anxiety baselines
6. **Longitudinal learning**: Refine parameters over time based on outcomes

The system demonstrates how **quantitative neuroscience research can be translated into algorithmic models** that derive meaningful biological insights from everyday activity data.