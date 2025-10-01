# Glassflow - Design Reference

## 🎨 Color Palette

### Primary Colors (Monochrome)
```
Black       #000000  rgb(0, 0, 0)         Background
White       #FFFFFF  rgb(255, 255, 255)   Primary text
```

### Gray Scale
```
Gray 900    #111827  rgb(17, 24, 39)      Darkest gray
Gray 800    #1f2937  rgb(31, 41, 55)      Dark gray
Gray 700    #374151  rgb(55, 65, 81)      Medium-dark gray
Gray 600    #4b5563  rgb(75, 85, 99)      Medium gray
Gray 500    #6b7280  rgb(107, 114, 128)   Base gray
Gray 400    #9ca3af  rgb(156, 163, 175)   Light-medium gray
Gray 300    #d1d5db  rgb(209, 213, 219)   Light gray
Gray 200    #e5e7eb  rgb(229, 231, 235)   Lighter gray
Gray 100    #f3f4f6  rgb(243, 244, 246)   Lightest gray
```

### Glass Blue (Brand Color - Use Sparingly!)
```
Blue 900    #001433  rgb(0, 20, 51)       Darkest blue
Blue 800    #002966  rgb(0, 41, 102)      Dark blue
Blue 700    #003d99  rgb(0, 61, 153)      Medium-dark blue
Blue 600    #0052cc  rgb(0, 82, 204)      Medium blue
Blue 500    #0066ff  rgb(0, 102, 255)     PRIMARY BRAND
Blue 400    #3385ff  rgb(51, 133, 255)    Light-medium blue
Blue 300    #66a3ff  rgb(102, 163, 255)   Light blue
Blue 200    #99c2ff  rgb(153, 194, 255)   Lighter blue
Blue 100    #cce0ff  rgb(204, 224, 255)   Lightest blue
Blue 50     #e6f0ff  rgb(230, 240, 255)   Palest blue
```

## ✨ Visual Effects

### Glass Effect
```css
.glass-effect {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.5rem;
}
```

**Usage**: All card-like surfaces, containers, panels

### Glass Blue Glow
```css
.glass-blue-glow {
  box-shadow: 
    0 0 20px rgba(0, 102, 255, 0.3),
    0 0 40px rgba(0, 102, 255, 0.2),
    inset 0 0 20px rgba(0, 102, 255, 0.1);
}
```

**Usage**: Active elements, focused inputs, brand highlights

### White Glow
```css
.white-glow {
  box-shadow: 
    0 0 20px rgba(255, 255, 255, 0.3),
    0 0 40px rgba(255, 255, 255, 0.2);
}
```

**Usage**: Hover states, secondary highlights

### Text Shimmer
```css
.text-shimmer {
  background: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0.8) 0%,
    rgba(255, 255, 255, 1) 50%,
    rgba(255, 255, 255, 0.8) 100%
  );
  background-size: 200% auto;
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: shimmer 3s linear infinite;
}
```

**Usage**: Headings, important text, loading states

### Border Shimmer
```css
.border-shimmer {
  background: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0.1) 0%,
    rgba(255, 255, 255, 0.3) 50%,
    rgba(255, 255, 255, 0.1) 100%
  );
  background-size: 200% auto;
  animation: shimmer 3s linear infinite;
}
```

**Usage**: Active borders, loading indicators

### Pulse Effect
```css
.pulse-slow {
  animation: pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
```

**Usage**: Thinking/processing states, attention grabbers

## 🎭 Component States

### Agent Status Colors
```
idle:      text-gray-400    #9ca3af
thinking:  text-blue-400    #3385ff  + pulse + glow
executing: text-blue-500    #0066ff  + glow
complete:  text-white       #FFFFFF
error:     text-red-400     #f87171
```

### Interactive States
```
Default:   glass-effect
Hover:     glass-effect + white-glow
Focus:     glass-effect + glass-blue-glow + shimmer
Active:    glass-effect + glass-blue-glow + scale(1.01)
Disabled:  glass-effect + opacity-50
```

## 📐 Spacing System

### Padding
```
p-1    0.25rem   4px
p-2    0.5rem    8px
p-3    0.75rem   12px
p-4    1rem      16px
p-6    1.5rem    24px
p-8    2rem      32px
p-12   3rem      48px
```

### Margins
```
m-1    0.25rem   4px
m-2    0.5rem    8px
m-3    0.75rem   12px
m-4    1rem      16px
m-6    1.5rem    24px
m-8    2rem      32px
```

### Gaps (Flexbox/Grid)
```
gap-2   0.5rem    8px
gap-3   0.75rem   12px
gap-4   1rem      16px
gap-6   1.5rem    24px
gap-8   2rem      32px
```

## 📝 Typography

### Font Families
```css
/* System Font Stack */
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 
             'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 
             'Fira Sans', 'Droid Sans', 'Helvetica Neue', 
             sans-serif;

/* Monospace (for code/commands) */
font-family: source-code-pro, Menlo, Monaco, Consolas, 
             'Courier New', monospace;
```

### Font Sizes
```
text-xs     0.75rem   12px
text-sm     0.875rem  14px
text-base   1rem      16px
text-lg     1.125rem  18px
text-xl     1.25rem   20px
text-2xl    1.5rem    24px
text-3xl    1.875rem  30px
```

### Font Weights
```
font-normal    400
font-semibold  600
font-bold      700
```

### Line Heights
```
leading-tight   1.25
leading-normal  1.5
leading-relaxed 1.625
```

## 🎬 Animation Timings

### Durations
```
Fast:     0.15s - 0.2s   (micro-interactions)
Normal:   0.3s - 0.5s    (standard transitions)
Slow:     0.7s - 1s      (dramatic entrances)
Very Slow: 2s - 3s       (ambient animations)
```

### Easing Functions
```
ease-in:        cubic-bezier(0.4, 0, 1, 1)
ease-out:       cubic-bezier(0, 0, 0.2, 1)
ease-in-out:    cubic-bezier(0.4, 0, 0.2, 1)
linear:         linear
```

### Common Animations
```
Fade In:        opacity: 0 → 1 (0.5s)
Slide Up:       y: 20px → 0 (0.5s)
Slide Down:     y: -20px → 0 (0.5s)
Scale Up:       scale: 0.9 → 1 (0.5s)
Shimmer:        background-position (3s infinite)
Pulse:          opacity: 1 → 0.5 → 1 (3s infinite)
Glow:           box-shadow intensity (2s alternate)
```

## 🎯 Usage Guidelines

### DO ✅
- Use monochrome (black/white/gray) as the base
- Use glass blue ONLY for branding and key accents
- Apply glass effects to all surfaces
- Use glowing effects on active/focused elements
- Animate state changes smoothly
- Keep text readable with good contrast
- Use shimmer for loading/processing states

### DON'T ❌
- Don't use glass blue everywhere
- Don't mix other colors into the palette
- Don't use harsh borders (use glass effect instead)
- Don't skip animations on state changes
- Don't use low-contrast text
- Don't overuse glow effects
- Don't create cluttered layouts

## 📦 Component Examples

### Button (Primary)
```tsx
<GlassSurface 
  className="px-6 py-3 cursor-pointer"
  glow={true}
  glowColor="blue"
  shimmer={true}
>
  <span className="text-white font-semibold">Action</span>
</GlassSurface>
```

### Button (Secondary)
```tsx
<GlassSurface 
  className="px-6 py-3 cursor-pointer"
  glow={false}
>
  <span className="text-gray-300 font-semibold">Cancel</span>
</GlassSurface>
```

### Card
```tsx
<GlassSurface className="p-6">
  <h3 className="text-lg font-semibold text-white mb-2">Title</h3>
  <p className="text-gray-400">Content</p>
</GlassSurface>
```

### Input
```tsx
<GlassSurface 
  glow={isFocused}
  glowColor="blue"
  shimmer={isFocused}
  className="p-1"
>
  <input 
    className="bg-transparent text-white outline-none px-4 py-2"
    onFocus={() => setIsFocused(true)}
    onBlur={() => setIsFocused(false)}
  />
</GlassSurface>
```

### Status Badge
```tsx
<span className="text-xs font-semibold text-glass-blue-500 px-3 py-1 rounded-full glass-effect">
  Active
</span>
```

## 🎨 Inspiration & References

### Design Philosophy
- **Minimal**: Less is more
- **Monochrome**: Pure black & white base
- **Glass**: Transparency and blur
- **Glow**: Soft, ethereal lighting
- **Blue Accent**: Strategic brand color
- **Smooth**: Fluid animations
- **Clean**: Polished and professional

### Visual Style
- Inspired by: Glass morphism, Neumorphism, Modern UI
- Mood: Professional, futuristic, clean, sophisticated
- Feel: Smooth, polished, premium, intelligent

## 🔧 Tailwind Classes Quick Reference

### Most Used Classes
```
bg-black                  Black background
text-white                White text
text-gray-400             Gray text
text-glass-blue-500       Brand blue text
glass-effect              Glass surface
glass-blue-glow           Blue glow effect
text-shimmer              Shimmer text
animate-pulse-slow        Slow pulse
rounded-lg                Large border radius
p-4                       Padding 1rem
space-y-4                 Vertical spacing
flex items-center         Flex center
grid grid-cols-3          3-column grid
```

## 📱 Responsive Breakpoints

```
sm:   640px   @media (min-width: 640px)
md:   768px   @media (min-width: 768px)
lg:   1024px  @media (min-width: 1024px)
xl:   1280px  @media (min-width: 1280px)
2xl:  1536px  @media (min-width: 1536px)
```

### Usage
```tsx
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
  {/* 1 column on mobile, 2 on tablet, 3 on desktop */}
</div>
```

---

**Remember**: The design is minimal and monochrome with strategic glass blue accents. Every element should feel polished, smooth, and professional.
