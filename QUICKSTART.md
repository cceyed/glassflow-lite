# Glassflow - Quick Start Guide

## 🚀 Get Running in 2 Minutes

### Prerequisites
- Node.js 18+ installed
- npm installed
- (Optional) Rust 1.70+ for full Tauri build

### Step 1: Install Dependencies
```bash
cd glassflow-lite
npm install
```

### Step 2: Start Development Server
```bash
npm run dev
```

### Step 3: Open in Browser
Open http://localhost:1420

That's it! You should see the Glassflow welcome screen.

## 🎯 Try It Out

1. **Type a command** in the input field (e.g., "analyze project structure")
2. **Press Enter** to submit
3. **Watch the agents** orchestrate in real-time:
   - Planner thinks and creates a plan
   - Executor runs the tasks
   - Validator checks the results
4. **See the history** after completion

## 🎨 What You're Seeing

### Design Elements
- **Monochrome Base**: Pure black background with white text
- **Glass Blue Accents**: Strategic blue highlights for branding
- **Glass Morphism**: Blurred, transparent surfaces
- **Glowing Effects**: Pulsing blue glows on active elements
- **Shimmer Animations**: Gradient animations on text and borders

### Components
- **Logo**: Animated glass prism with gradient text
- **Command Input**: Glowing input field with blue accent
- **Agent Cards**: Real-time status and reasoning display
- **History**: Previous commands and their status

## 📁 Project Structure (Simplified)

```
src/
├── components/          # React components
│   ├── GlassSurface.tsx    # Reusable glass effect
│   ├── CommandInput.tsx    # Command input
│   ├── AgentCard.tsx       # Agent display
│   ├── AgentOrchestrator.tsx # Agent logic
│   └── Logo.tsx            # Animated logo
├── App.tsx              # Main app
└── index.css            # Global styles

src-tauri/
└── src/
    └── main.rs          # Rust backend
```

## 🛠️ Common Tasks

### Run Full Tauri App
```bash
npm run tauri:dev
```
*Note: Requires Rust installed*

### Build for Production
```bash
npm run build              # Frontend only
npm run tauri:build        # Full app
```

### Check for Errors
```bash
npm run build              # TypeScript + Vite
```

## 🎨 Customize the Design

### Change Colors
Edit `tailwind.config.js`:
```javascript
colors: {
  'glass-blue': {
    500: '#0066ff',  // Change this
  },
}
```

### Modify Animations
Edit `tailwind.config.js`:
```javascript
animation: {
  'pulse-slow': 'pulse 3s ...',  // Adjust timing
}
```

### Add New Effects
Edit `src/index.css`:
```css
@layer utilities {
  .your-effect {
    /* Your styles */
  }
}
```

## 🤖 Modify Agent Behavior

### Change Agent Types
Edit `src/components/AgentOrchestrator.tsx`:
```typescript
const initialAgents: Agent[] = [
  { id: "planner", name: "Planner Agent", ... },
  { id: "executor", name: "Executor Agent", ... },
  { id: "validator", name: "Validator Agent", ... },
  // Add more agents here
];
```

### Adjust Timing
Edit `src/components/AgentOrchestrator.tsx`:
```typescript
await delay(2000);  // Change delay times
```

### Add Reasoning Steps
```typescript
reasoning: [
  "Your custom reasoning step",
  "Another step",
]
```

## 🐛 Troubleshooting

### Port 1420 Already in Use
```bash
# Kill the process
lsof -ti:1420 | xargs kill -9

# Or change port in vite.config.ts
port: 1421,  // Use different port
```

### Build Fails
```bash
# Clean and reinstall
rm -rf node_modules dist
npm install
npm run build
```

### Styles Not Updating
```bash
# Restart dev server
# Ctrl+C to stop
npm run dev
```

### TypeScript Errors
```bash
# Check for errors
npm run build

# Most common: unused imports
# Remove unused imports from files
```

## 📚 Learn More

- **Full Documentation**: See `README.md`
- **Development Guide**: See `DEVELOPMENT.md`
- **Architecture**: See `ARCHITECTURE.md`
- **Project Summary**: See `PROJECT_SUMMARY.md`

## 🎯 Next Steps

1. **Explore the Code**: Start with `src/App.tsx`
2. **Modify an Agent**: Change reasoning steps
3. **Add a Feature**: Create a new component
4. **Customize Design**: Change colors or effects
5. **Build Something**: Use this as a template

## 💡 Tips

- **Hot Reload**: Changes auto-refresh in dev mode
- **Component Reuse**: Use `GlassSurface` for new components
- **Animations**: Framer Motion makes animations easy
- **Tailwind**: Use utility classes for quick styling
- **TypeScript**: Strict mode catches errors early

## 🌟 Key Features to Explore

1. **GlassSurface Props**:
   ```tsx
   <GlassSurface 
     glow={true} 
     glowColor="blue"
     shimmer={true}
     pulse={true}
   >
     Content
   </GlassSurface>
   ```

2. **Framer Motion**:
   ```tsx
   <motion.div
     initial={{ opacity: 0 }}
     animate={{ opacity: 1 }}
     transition={{ duration: 0.5 }}
   >
     Content
   </motion.div>
   ```

3. **Tailwind Classes**:
   ```tsx
   className="glass-effect glass-blue-glow text-shimmer"
   ```

## 🚀 Ready to Build?

You now have everything you need to:
- Understand the project structure
- Run the application
- Make modifications
- Build new features

**Happy coding!** 🎉
