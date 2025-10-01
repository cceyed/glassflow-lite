# Development Guide

## Quick Start

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Run development server** (frontend only):
   ```bash
   npm run dev
   ```
   Open http://localhost:1420 in your browser.

3. **Run Tauri app** (full desktop app):
   ```bash
   npm run tauri:dev
   ```

4. **Build for production**:
   ```bash
   npm run tauri:build
   ```

## Project Structure

```
glassflow-lite/
├── src/                      # React frontend
│   ├── components/          # React components
│   │   ├── GlassSurface.tsx    # Reusable glass morphism component
│   │   ├── CommandInput.tsx    # Command input with glowing effects
│   │   ├── AgentCard.tsx       # Individual agent display
│   │   ├── AgentOrchestrator.tsx # Agent orchestration logic
│   │   └── Logo.tsx            # Animated logo
│   ├── App.tsx              # Main application
│   ├── main.tsx             # React entry point
│   └── index.css            # Global styles with Tailwind
├── src-tauri/               # Rust backend
│   ├── src/
│   │   └── main.rs          # Tauri backend with agent commands
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
├── index.html               # HTML entry point
├── vite.config.ts           # Vite configuration
├── tailwind.config.js       # Tailwind configuration
└── package.json             # Node dependencies

```

## Key Features

### 1. GlassSurface Component
Reusable component with glass morphism effects:
- `glow`: Enable glow effect
- `glowColor`: "white" or "blue"
- `shimmer`: Enable shimmer animation
- `pulse`: Enable pulsing animation

### 2. Agent Orchestration
The `AgentOrchestrator` component manages:
- Multiple agents (Planner, Executor, Validator)
- Real-time status updates
- Reasoning step visualization
- Sequential agent execution

### 3. Command System
Commands are processed through:
1. Frontend: `CommandInput` component
2. State management: React hooks
3. Backend: Tauri commands (future integration)

## Design System

### Colors
- **Black**: `#000000` - Background
- **White**: `#FFFFFF` - Primary text
- **Gray**: Various shades for secondary text
- **Glass Blue**: `#0066ff` - Branding only

### Effects
- **Glass Effect**: Blur + transparency
- **Glow**: Box shadow with blue/white
- **Pulse**: Slow opacity animation
- **Shimmer**: Linear gradient animation

## Next Steps

1. **Integrate Real AI**: Replace mock orchestration with actual AI agents
2. **Add More Commands**: Expand command vocabulary
3. **Persist History**: Save command history to disk
4. **Add Settings**: User preferences and configuration
5. **Enhance Agents**: Add more specialized agent types

## Troubleshooting

### Icons Missing
Generate icons using:
```bash
npm run tauri icon path/to/icon.png
```

### Build Fails
Ensure you have:
- Node.js 18+
- Rust 1.70+
- Platform-specific Tauri dependencies

### Hot Reload Not Working
Try:
```bash
rm -rf node_modules dist
npm install
npm run dev
```
