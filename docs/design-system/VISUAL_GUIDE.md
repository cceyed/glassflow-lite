# Glassflow - Visual Guide

## 🎨 What You'll See

This guide describes the visual appearance of Glassflow when running.

---

## 🏠 Welcome Screen

### Layout
```
┌─────────────────────────────────────────────────────────┐
│                                                          │
│  [Glass Prism Icon]  Glassflow                          │  ← Logo (top left)
│                      AI ORCHESTRATION                    │     with blue glow
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │  › Enter your command to orchestrate agents... │    │  ← Command Input
│  └────────────────────────────────────────────────┘    │     (glows blue on focus)
│                                                          │
│         ┌──────────────────────────────────┐           │
│         │                                   │           │
│         │   Welcome to Glassflow            │           │  ← Welcome Card
│         │   ═══════════════════             │           │     (shimmer text)
│         │                                   │           │
│         │   A sophisticated command-driven  │           │
│         │   AI IDE with intelligent         │           │
│         │   multi-agent orchestration       │           │
│         │                                   │           │
│         │   ▸ Intelligent Agents:           │           │
│         │     Multiple specialized agents   │           │
│         │     work together...              │           │
│         │                                   │           │
│         │   ▸ Real-time Reasoning:          │           │
│         │     Watch agents think...         │           │
│         │                                   │           │
│         │   ▸ Command-Driven:               │           │
│         │     Simple, powerful commands...  │           │
│         │                                   │           │
│         └──────────────────────────────────┘           │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Visual Effects
- **Background**: Pure black (#000000)
- **Logo**: Glass prism with pulsing blue glow
- **Logo Text**: White to blue gradient
- **Command Input**: Glass surface, blue glow on focus
- **Welcome Card**: Glass surface with subtle border
- **Title**: Shimmer animation (white gradient sweep)
- **Bullet Points**: Glass blue arrows (▸)

---

## 🤖 Active Orchestration

### Layout
```
┌─────────────────────────────────────────────────────────┐
│                                                          │
│  [Glass Prism]  Glassflow                               │
│                 AI ORCHESTRATION          ● System Ready │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │  › analyze project structure                   │    │  ← Command Input
│  └────────────────────────────────────────────────┘    │
│                                                          │
│              Planning complete. Executing...            │  ← Phase Status
│                                                          │     (shimmer text)
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Planner      │  │ Executor     │  │ Validator    │ │
│  │ ────────     │  │ ────────     │  │ ────────     │ │
│  │ complete     │  │ executing    │  │ idle         │ │  ← Agent Cards
│  │              │  │              │  │              │ │     (with glow)
│  │ ▸ Breaking   │  │ ▸ Executing  │  │              │ │
│  │   down...    │  │   planned... │  │              │ │
│  │ ▸ Identifying│  │ ▸ Processing │  │              │ │
│  │   deps...    │  │   command... │  │              │ │
│  │ ▸ Plan       │  │              │  │              │ │
│  │   created    │  │              │  │              │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Visual Effects
- **Phase Status**: Large shimmer text
- **Agent Cards**: Glass surfaces
  - **Idle**: Gray text, no glow
  - **Thinking**: Blue text, pulse effect, blue glow
  - **Executing**: Bright blue text, blue glow
  - **Complete**: White text, no glow
- **Reasoning Steps**: Animated slide-in from left
- **Blue Arrows**: Glass blue color (#0066ff)

---

## 📜 Command History

### Layout
```
┌─────────────────────────────────────────────────────────┐
│                                                          │
│  [Glass Prism]  Glassflow                               │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │  › Enter your command to orchestrate agents... │    │
│  └────────────────────────────────────────────────┘    │
│                                                          │
│  Recent Commands                                        │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │ analyze project structure          ✓ Complete  │    │
│  │ 4:23:15 PM                                      │    │
│  └────────────────────────────────────────────────┘    │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │ create new component               ✓ Complete  │    │
│  │ 4:20:42 PM                                      │    │
│  └────────────────────────────────────────────────┘    │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │ refactor authentication            ✓ Complete  │    │
│  │ 4:15:33 PM                                      │    │
│  └────────────────────────────────────────────────┘    │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Visual Effects
- **History Items**: Glass surfaces, slide in from left
- **Command Text**: White monospace font
- **Timestamps**: Gray, small font
- **Status**: Green for complete, blue for processing
- **Checkmark**: Green color

---

## 🎭 Interactive States

### Command Input

**Default State**
```
┌────────────────────────────────────────────────┐
│  › Enter your command to orchestrate agents... │
└────────────────────────────────────────────────┘
```
- Glass surface
- Subtle white border
- Gray placeholder text

**Focused State**
```
┌════════════════════════════════════════════════┐  ← Blue glow
║  › |                                           ║  ← Shimmer border
└════════════════════════════════════════════════┘
```
- Blue glow effect
- Shimmer border animation
- Slightly scaled up (1.01)
- White cursor

**With Text**
```
┌────────────────────────────────────────────────┐
│  › analyze project structure                   │
└────────────────────────────────────────────────┘
```
- White text
- Monospace font

---

### Agent Cards

**Idle State**
```
┌──────────────┐
│ Validator    │
│ ────────     │
│ idle         │  ← Gray text
│              │
└──────────────┘
```
- Glass surface
- Gray status text
- No glow

**Thinking State**
```
┌══════════════┐  ← Blue glow
║ Planner      ║  ← Pulsing
║ ────────     ║
║ thinking     ║  ← Blue text
║              ║
║ ▸ Breaking   ║
║   down...    ║
└══════════════┘
```
- Blue glow
- Pulse animation
- Blue status text
- Reasoning steps appear

**Executing State**
```
┌══════════════┐  ← Bright blue glow
║ Executor     ║
║ ────────     ║
║ executing    ║  ← Bright blue
║              ║
║ ▸ Executing  ║
║   planned... ║
║ ▸ Processing ║
║   command... ║
└══════════════┘
```
- Bright blue glow
- Bright blue text
- Multiple reasoning steps

**Complete State**
```
┌──────────────┐
│ Planner      │
│ ────────     │
│ complete     │  ← White text
│              │
│ ▸ Breaking   │
│   down...    │
│ ▸ Identifying│
│   deps...    │
│ ▸ Plan       │
│   created    │  ← Final step
└──────────────┘
```
- No glow
- White status text
- All reasoning steps visible

---

## 🌟 Animation Sequences

### Page Load
```
0ms:    Black screen
100ms:  Logo fades in + slides down
300ms:  Logo glow starts pulsing
500ms:  Command input fades in + slides up
700ms:  Welcome card fades in + slides up
900ms:  Welcome title shimmer starts
```

### Command Submission
```
0ms:    User presses Enter
50ms:   Welcome card fades out
100ms:  Phase status fades in
200ms:  Agent cards appear (staggered)
        - Planner at 200ms
        - Executor at 300ms
        - Validator at 400ms
```

### Agent Orchestration
```
0ms:    "Analyzing command..."
1000ms: Planner starts thinking (blue glow + pulse)
        First reasoning step appears
1500ms: Second reasoning step appears
2000ms: Third reasoning step appears
3000ms: Planner completes (glow fades)
3500ms: "Planning complete. Executing..."
        Executor starts (blue glow)
4000ms: Executor reasoning appears
6000ms: Executor completes
6500ms: "Execution complete. Validating..."
        Validator starts (blue glow + pulse)
8500ms: Validator completes
9000ms: "Task complete!"
9500ms: History view fades in
```

### Shimmer Effect
```
Continuous loop:
0.0s: Gradient at left edge
0.5s: Gradient at 25%
1.0s: Gradient at 50% (brightest)
1.5s: Gradient at 75%
2.0s: Gradient at right edge
2.5s: Gradient at 125%
3.0s: Back to start (seamless loop)
```

### Pulse Effect
```
Continuous loop:
0.0s: Opacity 1.0, scale 1.0
1.0s: Opacity 0.5, scale 0.98
2.0s: Opacity 1.0, scale 1.0
3.0s: Opacity 0.5, scale 0.98
(repeats)
```

---

## 🎨 Color Examples

### Text Colors in Context

**Primary Text** (White #FFFFFF)
```
Glassflow
Planner Agent
analyze project structure
```

**Secondary Text** (Gray #9ca3af)
```
AI ORCHESTRATION
Recent Commands
4:23:15 PM
```

**Glass Blue** (#0066ff)
```
› (command prompt)
▸ (bullet points)
thinking (status when active)
```

**Status Colors**
```
✓ Complete (Green #4ade80)
Processing... (Blue #3385ff)
● System Ready (Blue #0066ff)
```

---

## 💫 Special Effects

### Glass Morphism
- Semi-transparent background
- 10px blur
- Subtle white border
- Slight inner shadow
- Used on all cards and surfaces

### Glow Effects

**Blue Glow** (Active elements)
- Outer glow: 20px, blue, 30% opacity
- Mid glow: 40px, blue, 20% opacity
- Inner glow: 20px, blue, 10% opacity

**White Glow** (Hover states)
- Outer glow: 20px, white, 30% opacity
- Mid glow: 40px, white, 20% opacity

### Shimmer
- Linear gradient sweep
- White highlight at center
- 3 second loop
- Smooth, continuous motion

### Pulse
- Slow opacity fade
- Subtle scale change
- 3 second cycle
- Ease-in-out timing

---

## 📐 Spacing & Layout

### Padding
- Cards: 24px (1.5rem)
- Buttons: 16px horizontal, 12px vertical
- Input: 16px all sides
- Containers: 32px (2rem)

### Margins
- Between sections: 32px
- Between cards: 16px
- Between elements: 8px

### Border Radius
- Cards: 8px
- Buttons: 6px
- Input: 8px
- Badges: 999px (pill shape)

---

## 🖼️ Visual Hierarchy

### Size Hierarchy
1. **Logo** - Largest, most prominent
2. **Phase Status** - Large shimmer text
3. **Command Input** - Medium, centered
4. **Agent Cards** - Medium, grid layout
5. **History Items** - Small, list layout
6. **Timestamps** - Smallest, metadata

### Color Hierarchy
1. **White** - Most important (headings, active text)
2. **Glass Blue** - Brand accents, active states
3. **Gray** - Secondary information
4. **Dim Gray** - Metadata, timestamps

### Visual Weight
1. **Glowing elements** - Highest attention
2. **Pulsing elements** - Active processing
3. **Shimmer elements** - Important headings
4. **Static elements** - Standard content

---

## 🎬 User Journey

### First Launch
1. See logo with blue glow
2. Read welcome message
3. Focus on command input (blue glow appears)
4. Type first command
5. Press Enter

### Command Execution
1. Welcome fades out
2. Phase status appears
3. Agent cards slide in
4. Watch agents work (blue glows, pulsing)
5. See reasoning steps appear
6. Task completes
7. History appears

### Subsequent Commands
1. Type new command
2. History fades out
3. New orchestration begins
4. Previous pattern repeats

---

**This is the visual language of Glassflow**: Clean, minimal, monochrome with strategic glass blue accents. Every animation is smooth, every effect is polished, and the focus is always on the agents doing their work.
