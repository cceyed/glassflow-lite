PHASE 1: 

Foundation & Visual System

1.1 Visual Design System
Color Palette
Base Colors:
Background:     #000000  (pure black)
Surface:        #0f0f0f  (elevated black)
Border:         #1a1a1a  (subtle borders)
Divider:        #2a2a2a  (section separators)
Text Colors:
Primary:        #ffffff  (pure white, main text)
Secondary:      #b0b0b0  (silver, secondary text)
Tertiary:       #707070  (dim gray, metadata)
Disabled:       #404040  (very dim, disabled states)
Glass Accent (Branding ONLY):
Glass Base:     rgba(100, 150, 255, 0.08)
Glass Border:   rgba(100, 150, 255, 0.25)
Glass Glow:     rgba(100, 150, 255, 0.4)
Glass Bright:   rgba(100, 150, 255, 0.6)
Effect Colors:
Shimmer:        rgba(255, 255, 255, 0.6)
Glow White:     rgba(255, 255, 255, 0.3)
Selection:      rgba(255, 255, 255, 0.12)
Overlay:        rgba(0, 0, 0, 0.8)
Typography
Primary Font (UI):
Font Family: 'Inter', -apple-system, system-ui, sans-serif
Weights: 300, 400, 500, 600, 700
Monospace Font (Code/Terminal):
Font Family: 'JetBrains Mono', 'SF Mono', monospace
Weights: 400, 500, 600
Type Scale:
Hero:      48px / 700 (logo, splash screens)
Title:     32px / 600 (main headings)
Heading:   24px / 600 (section headers)
Large:     18px / 500 (emphasized text)
Body:      16px / 400 (standard text)
Small:     14px / 400 (secondary info)
Caption:   12px / 400 (metadata)
Tiny:      10px / 400 (timestamps)
Effect System
Shimmer Animation:

Diagonal gradient sweep (45deg)
White highlight at 50%
2.5s duration, infinite loop
Used on: Loading states, brand elements

Glow Effect:

Multi-layer box-shadow
Inner and outer glow
Pulsing opacity variation
Used on: Active elements, focus states

Pulse Animation:

Scale: 1.0 → 1.02 → 1.0
Opacity: 1.0 → 0.85 → 1.0
2s duration, ease-in-out
Used on: Active agents, processing states

Glass Effect:

Background blur: 24px
Semi-transparent blue tint
Bright border with inner shadow
Subtle glow around edges
Used on: Logo, brand accents ONLY

Layout System
Window:

Size: 1400x900 default
Min size: 1200x750
Frameless with custom titlebar
No decorations, pure black chrome

Grid:

8px base unit
All spacing multiples of 8
Consistent padding/margin system

Main Layout:
┌─────────────────────────────────────────┐
│  Titlebar (40px)                        │
├─────────────────────────────────────────┤
│  Command Input (60px)                   │
├─────────────────────────────────────────┤
│                                          │
│  Content Area (flexible)                │
│                                          │
├─────────────────────────────────────────┤
│  Agent Status Bar (48px)                │
├─────────────────────────────────────────┤
│  Status Bar (32px)                      │
└─────────────────────────────────────────┘
1.2 Component Library
TitleBar

Height: 40px
Left: Glassflow logo (glass effect)
Center: Current project name
Right: Window controls
Background: Pure black
Bottom border: #1a1a1a

CommandInput

Height: 60px
Placeholder: "Describe what you want to build..."
Font: 18px, weight 400
White text on black
Subtle border bottom
Shimmer on focus
Submit: Enter key

AgentStatusBar

Height: 48px
4 agent indicators in row
Each shows: Name, Status LED, Progress
Separator: 1px vertical lines
Hover: Expand reasoning panel
Background: #0f0f0f

StatusBar

Height: 32px
Left: Project path
Center: Phase indicator
Right: Time elapsed
Background: #000000
Top border: #1a1a1a

Button

Variants: Primary, Secondary, Ghost
States: Default, Hover, Active, Disabled
Primary: White bg, black text
Secondary: Border, white text
Hover: Shimmer effect
Active: Slight scale down

Card

Background: #0f0f0f
Border: 1px solid #1a1a1a
Border radius: 8px
Padding: 24px
Hover: Border brightens
Glow on active

Modal

Overlay: rgba(0, 0, 0, 0.9)
Content: Card with glow
Center screen
Backdrop blur: 10px
Scale animation in/out