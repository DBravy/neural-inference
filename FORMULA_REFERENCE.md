# Research Formula Quick Reference

## Sleep

### Adenosine Clearance
```
A_after = A_before × e^(-duration_hours / 7.5) × quality × 0.85
Half-life per cycle: ~90 minutes
```

### Dopamine Restoration
```
If duration ≥ 7h: +0.3 × quality
If 6-7h: +0.15 × quality
If <6h: -0.2 × (1 - quality)  // Deprivation penalty
```

### Glucose Metabolic Threshold
```
If duration ≥ 6h AND efficiency ≥ 0.67: +0.2
If <6h: -0.3 × (1 - duration/6)  // 40% impairment
```

### Cortisol
```
If quality ≥ 0.7: -0.2
Else: +0.15 × (1 - quality)  // Evening elevation
```

---

## Light Exposure

### Circadian Phase Shift
```
If intensity ≥ 2000 lux:
  Morning (6-11am): -0.8h × (duration/120min) × adenosine_gating
  Evening (6-10pm): +0.6h × (duration/120min)
  Daytime: -0.2h (stabilizes)
If 100-2000 lux:
  Morning: -0.3h
  Evening: +0.3h

Adenosine gating:
  If adenosine > 0.7: 0.5 (50% sensitivity)
  If adenosine 0.5-0.7: 0.75
  If adenosine < 0.5: 1.0
```

### Serotonin (Morning Light)
```
If morning AND intensity ≥ 2000 lux:
  +0.25 × (intensity/10000) × (duration/30min)
```

### Cortisol (Morning Light)
```
If morning AND timing ≤ 2h after wake:
  5000+ lux: +0.2
  800-5000 lux: +0.15
  <800 lux: +0.05
```

---

## Nutrition

### Glucose Response
```
If protein ≥ 50g:
  0.3 + GI_score × 0.3  // Protein dampens
Else:
  0.4 + GI_score × 0.5  // Higher response

GI_score: low=0.3, medium=0.6, high=1.0
```

### Serotonin (Carb-Protein Ratio)
```
If protein% < 10 AND carbs > 40g:
  +0.35 × (1 - protein%/20)  // Maximized
If protein% > 25:
  -0.15  // Blocked by high protein
Else:
  +0.1
```

### Dopamine (Protein + Reward)
```
Tyrosine boost: 0.15 × (protein/50g)  // Saturates at 50g
GI reward: GI_score × 0.1  // High-GI more rewarding
Total: tyrosine_boost + GI_reward
```

---

## Caffeine

### Receptor Occupancy
```
A2A: plasma_conc / (plasma_conc + 65mg)
A1: plasma_conc / (plasma_conc + 450mg)

Plasma concentration: dose_mg × e^(-0.15 × hours_ago)
Half-life: 5 hours
```

### Primitive Impacts
```
Adenosine blockade: -0.5 × A2A_occupancy
Dopamine: +0.15 × (dose/200mg)
Norepinephrine: +0.25 × (dose/200mg)
Cortisol: +0.2 × (dose/200mg)  // Tolerance develops in 5 days
```

### Circadian Phase Delay
```
If hours_before_sleep ≤ 6:
  delay = 0.3 × (dose/200mg) × (1 - hours/6)
```

---

## Exercise

### Intensity Mapping
```
light: 40% VO2max
moderate: 65% VO2max
vigorous: 75% VO2max
high_intensity: 85% VO2max
```

### Dopamine
```
If HIIT: 0.35 × (duration/30min)
If intensity ≥ 70%: 0.3 × (duration/45min)
Else: 0.2 × (duration/45min)
```

### Cortisol (Threshold at 60% VO2max)
```
If intensity ≥ 80%: 0.4 × (duration/30min)
If 60-80%: 0.2 × (duration/45min)
If <40%: -0.1  // Light exercise reduces cortisol
```

### Glucose Depletion
```
-0.3 × (intensity/60)^2 × (duration/60min)
Rate ∝ Intensity^2.5
```

### Circadian Phase
```
Morning exercise: -0.3h (advance)
Evening exercise: 0h (no effect)
```

---

## Naps

### Adenosine Clearance
```
A_after = A_before × e^(-0.0077 × duration_min)

20min nap: 14% clearance
90min nap: 50% clearance
```

### Circadian Phase
```
Morning: +0.2h × (duration/60min)  // Delay
Afternoon (1-3pm): 0h  // Safe window
Evening: -0.3h × (duration/60min)  // Advance
```

---

## Stress

### Cortisol Multiplier
```
uncontrollable + social-evaluative: 3.0×
uncontrollable only: 2.0×
social-evaluative only: 1.5×
controllable: 0.5×

cortisol = 0.3 × intensity × multiplier × (duration/45min)
Peak: 20-30min, Recovery: 40-60min
```

### Norepinephrine
```
If controllable: 0.3 × intensity × (duration/30min)
Else: 0.15 × intensity
```

### Dopamine
```
If controllable: +0.1 × intensity  // Enhancement
Else: -0.2 × intensity  // Depletion
```

---

## Social Interaction

### Dopamine (Interaction Type)
```
Reciprocal: 1.0× (baseline)
Unilateral: 0.79× (42/53 neurons)
Passive: 0.13× (7/53 neurons)

Quality multiplier:
  positive: 1.5×
  neutral: 1.0×
  negative: 0.3×

Novelty boost: 1.3× for novel interactions

dopamine = 0.25 × quality × interaction_type × novelty × (duration/60min)
Peak at 27% of interaction duration
```

### Serotonin
```
If positive: +0.2 × interaction_type × (duration/60min)
If negative: -0.1
Else: +0.05
```

### Cortisol
```
If negative: +0.3  // Rejection response
If positive: -0.15  // Stress buffering
Else: 0
```

---

## Screen Time

### Circadian Phase Delay
```
If hours_before_sleep ≤ 3:
  Blue light intensity: low=0.3, medium=0.6, high=1.0
  delay = 0.5 × intensity × (duration/90min) × (1 - hours/3)
```

### Dopamine (Content Type)
```
social_media: 0.2 × (duration/60min)
gaming: 0.25 × (duration/60min)
work: 0.05
passive: 0
```

---

## Interruptions

### Per Interruption
```
Cortisol: +0.05 × frequency  // ~2× vs uninterrupted
Norepinephrine: +0.067 × frequency
Glucose: -0.033 × frequency
Recovery time: 23 minutes per interruption
```

---

## Sequence Patterns

### Chronic Sleep Deprivation
```
Detection: 3+ poor-quality sleep events within 72h
Effects:
  Dopamine: -0.25
  Serotonin: -0.20
```

### Caffeine Withdrawal
```
Detection: 7+ events (≥100mg/day) → 24-168h gap
Intensity: 1.0 at 48h, then e^(-0.02×(hours-48))
Effects:
  Dopamine: -0.20 × intensity
  Serotonin: -0.15 × intensity
  Norepinephrine: -0.20 × intensity
  Cortisol: +0.15 × intensity
```

### Late Caffeine → Sleep Disruption
```
Detection: Caffeine within 9h of sleep + poor quality
Severity: (dose/100mg) × (1 - hours_before/9)
Effect: Cortisol +0.1 to +0.2
```

### Quality Sleep → Morning Exercise
```
Detection: Good sleep (≥7h) → exercise within 6h
Effect: Dopamine +0.15 (synergy)
```

---

## Dopamine-Serotonin Balance (NEW)

### Dual Time Scale Computation

**Both dopamine and serotonin are now computed with TWO time scales:**

1. **Acute Score** (Recent state - what you feel NOW)
   - Dopamine: 12-hour window, 6-hour decay half-life
   - Serotonin: 16-hour window, 8-hour decay half-life
   - Captures immediate effects of recent events

2. **Chronic Score** (Baseline state - your underlying condition)
   - Dopamine: 72-hour window, 24-hour decay half-life
   - Serotonin: 96-hour window, 36-hour decay half-life
   - Captures sustained patterns and depletion/buildup

**Combined Score Formula:**
```
combined_score = 0.7 × acute_score + 0.3 × chronic_score
```

This means recent events (last 12-16h) matter MORE than older events.

### Reciprocal Inhibition

**Research basis:** 5-HT2C receptors on VTA dopamine neurons inhibit DA release. High serotonin suppresses dopamine more strongly than vice versa.

**Serotonin inhibits Dopamine:**
```
IF serotonin > 0.6:
    inhibition_factor = 1.0 - 0.25 × ((serotonin - 0.6) / 0.4)
    dopamine_effective = dopamine_combined × inhibition_factor
ELSE:
    dopamine_effective = dopamine_combined
```

**Dopamine inhibits Serotonin (weaker):**
```
IF dopamine > 0.7:
    inhibition_factor = 1.0 - 0.15 × ((dopamine - 0.7) / 0.3)
    serotonin_effective = serotonin_combined × inhibition_factor
ELSE:
    serotonin_effective = serotonin_combined
```

### Dopamine/Serotonin Ratio

```
ratio = dopamine_effective / serotonin_effective
```

**Interpretation:**
- ratio > 1.4: **Dopamine-dominant** (High drive, lower mood stability)
- ratio 0.65-1.4: **Balanced** (Optimal flexibility and resilience)
- ratio < 0.65: **Serotonin-dominant** (Stable mood, lower motivation)

### Functional States

States are determined by BOTH the ratio AND absolute levels:

1. **Peak Performance** (Both ≥ 0.65)
   - High motivation + stable mood
   - Optimal for challenging tasks

2. **Depleted** (Both ≤ 0.40)
   - Low motivation + low mood stability
   - Need recovery

3. **Driven but Anxious** (DA ≥ 0.65, 5HT ≤ 0.45 OR ratio > 1.4)
   - High drive but reduced stress resilience
   - Burnout risk

4. **Calm but Unmotivated** (DA ≤ 0.45, 5HT ≥ 0.65 OR ratio < 0.65)
   - Stable mood but reduced initiative
   - Content but sluggish

5. **Balanced (DA-leaning)** (ratio 1.1-1.4)
   - Good motivation with reasonable stability
   - Slight bias toward drive

6. **Balanced (5HT-leaning)** (ratio 0.75-0.9)
   - Stable mood with moderate motivation
   - Slight bias toward calm

7. **Well-Balanced** (ratio 0.9-1.1)
   - Optimal balance
   - Flexible and resilient

---

## Cross-Primitive Modifiers

### Circadian Misalignment
```
If phase < 0.35 OR phase > 0.65:
  dopamine × 0.85
  serotonin × 0.85
  norepinephrine × 0.85
```

### Low Glucose
```
If glucose < 0.4:
  dopamine × 0.70
  serotonin × 0.70
  norepinephrine × 0.70
```

### High Adenosine
```
If adenosine > 0.7:
  impairment = 0.7 + 0.3 × ((adenosine - 0.7) / 0.3)
  dopamine × impairment  // A2A-D2 heteromers
```

### High Cortisol
```
If cortisol > 0.7:
  dopamine × 0.85
  serotonin × 0.85
```

---

## Time Decay

### Exponential Decay Formula
```
decay_factor = e^(-0.693 × hours_ago / half_life)

Half-lives:
  Glucose: 2h
  Norepinephrine: 4h
  Adenosine: 16h
  Cortisol: 12h
  Dopamine: 24h
  Serotonin: 36h
  Circadian Phase: 72h
```

---

## Adenosine Special (Process S)

### Wake Accumulation
```
base = 1 - e^(-hours_awake / 16)
Saturates at ~0.85 after 16+ hours awake
```

### Caffeine Blockade
```
total_blockade = Σ[dose_i × e^(-0.15×hours_ago_i) / (conc + 65)] × 0.5
Capped at 0.6
```

---

## Circadian Phase Special (Process C)

### Phase Score to Offset
```
phase_offset_hours = (phase_score - 0.5) × 10
Range: 0.3-0.7 → ±2 hours offset
```

### Circadian Sleep Propensity
```
adjusted_hour = (hour_of_day - phase_offset_hours + 24) % 24
raw_pressure = -cos(2π × (adjusted_hour - 3) / 24)
scaled = 0.3 + 0.4 × (raw_pressure + 1) / 2
Peak at 3am (biological night)
```

---

## Sleep Drive (Two-Process Model)

### Combined Formula
```
sleep_drive = 0.6 × adenosine + 0.4 × circadian_component

Interpretation:
  <0.3: Alert
  0.3-0.5: Comfortable wake
  0.5-0.7: Increasing pressure
  >0.7: Strong urge to sleep
```

---

## Quick Threshold Reference

| Metric | Threshold | Effect |
|--------|-----------|--------|
| Sleep duration | <6h | Metabolic impairment |
| Sleep efficiency | <67% | Reduced benefits |
| Light intensity | 100 lux | Minimum circadian |
| Light intensity | 2000 lux | Saturation point |
| Exercise intensity | 60% VO2max | Cortisol activation |
| Exercise intensity | 80% VO2max | Maximum effects |
| Caffeine dose | 65mg | 50% A2A occupancy |
| Caffeine dose | 450mg | 50% A1 occupancy |
| Caffeine timing | 6h before sleep | Sleep disruption |
| Protein % | <10% | Serotonin maximized |
| Protein % | >25% | Serotonin blocked |
| Protein amount | 15-20g | Adequate for NT synthesis |
| Protein amount | >50g | No additional benefit |
| Meal GI | <55 | Low (stable) |
| Meal GI | 55-70 | Medium |
| Meal GI | >70 | High (spike) |

---

This quick reference provides all key formulas for implementing research-based primitive estimation. For detailed explanations and research citations, see `doc.md`.