# Glassflow

A sophisticated multi-agent AI IDE with intelligent orchestration.

## 🎯 Overview

Glassflow uses 4 specialized AI agents working together to transform specifications into production-ready code:

1. **Architect Agent** (✅ Complete) - Analyzes specs, asks clarifying questions, designs architecture
2. **Planner Agent** (🚧 Coming Soon) - Breaks down architecture into actionable tasks
3. **Executor Agent** (🚧 Coming Soon) - Generates code based on the plan
4. **Validator Agent** (🚧 Coming Soon) - Validates code quality and runs tests

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Run development server
npm run tauri:dev
```

## 📁 Project Structure

```
glassflow-lite/
├── src/                      # Frontend (React + TypeScript)
│   ├── agents/              # Agent-specific components
│   │   └── architect/       # Architect agent UI
│   ├── design-system/       # UI design system
│   └── shared/              # Shared components
│
├── src-tauri/               # Backend (Rust + Tauri)
│   └── src/
│       ├── agents/          # Agent business logic
│       │   └── architect/   # Architect agent implementation
│       ├── ipc/             # IPC commands
│       ├── llm/             # LLM integration (OpenRouter)
│       ├── models/          # Data models
│       └── persistence/     # State persistence
│
└── docs/                    # Documentation
    ├── architecture/        # System architecture docs
    ├── agents/              # Agent-specific docs
    └── design-system/       # Design system docs
```

## 🏗️ Architecture

### Architect Agent (Current)

The Architect Agent is fully implemented with a complete state machine:

```
IDLE → ANALYZING → QUESTIONING → DESIGNING → COMPLETE
  ↑                                              ↓
  ← ← ← ← ← ← ← ERROR ← ← ← ← ← ← ← ← ← ← ← ← ←
```

**Features**:
- LLM-powered specification analysis
- Intelligent question generation for ambiguous specs
- Architecture plan generation with confidence scoring
- Session recovery (state persists across app restarts)
- Real-time event streaming to frontend

### Multi-Agent System (Planned)

```
User Spec
    ↓
Architect → Architecture Plan
    ↓
Planner → Task List
    ↓
Executor → Generated Code
    ↓
Validator → Validation Report
```

## 🎨 Design System

Glassflow features a custom design system with:
- Monochrome palette with strategic glass-blue accents
- 8px grid system
- Consistent spacing and typography
- Glass morphism effects
- Accessible components

See [docs/design-system/](./docs/design-system/) for details.

## 📚 Documentation

- [Architecture Overview](./docs/architecture/)
- [Architect Agent](./docs/agents/architect.md)
- [Design System](./docs/design-system/)
- [API Reference](./docs/api/)

## 🧪 Testing

```bash
# Run unit tests
npm test

# Run E2E tests
npm run test:e2e

# Run visual tests
npx playwright test
```

## 🛠️ Development

### Prerequisites
- Node.js 18+
- Rust 1.70+
- Tauri CLI

### Environment Variables
```bash
# Optional: Set custom OpenRouter API key
OPENROUTER_API_KEY=your_key_here
```

### Building
```bash
# Development build
npm run tauri:dev

# Production build
npm run tauri:build
```

## 📝 License

MIT

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines first.

---

**Status**: Architect Agent complete, 3 more agents in development.
