# Glassflow

A sophisticated multi-agent AI code generation system with intelligent orchestration.

## 🎯 Overview

Glassflow uses 4 specialized AI agents + 1 orchestrator working together to transform specifications into production-ready code:

1. **🏗️ Architect Agent** (✅ Complete) - Analyzes specs, asks clarifying questions, designs architecture
2. **⚙️ Engineer Agent** (✅ Complete) - Generates code files from architecture plans
3. **✅ Quality Agent** (✅ Complete) - Reviews code quality, detects issues, auto-fixes problems
4. **🐛 Debug Agent** (✅ Complete) - Tests runtime behavior, validates execution, fixes bugs
5. **🎬 Orchestrator** (✅ Complete) - Coordinates pipeline flow, manages retries, aggregates state

## 🚀 Quick Start

```bash
# 1. Install dependencies
npm install

# 2. Set up API key
cp .env.example .env
# Edit .env and add your OpenRouter API key

# 3. Run development server
npm run tauri dev

# 4. Validate system
bash scripts/validate-all.sh
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

### Complete Multi-Agent Pipeline

```
User Specification
        ↓
    🎬 Orchestrator (Coordinator)
        ↓
    🏗️ Architect → Architecture Plan
        ↓
    ⚙️ Engineer → Generated Code
        ↓
    ✅ Quality → Quality Report
        ↓
    🐛 Debug → Debug Report
        ↓
Production-Ready Code ✨
```

**Pipeline Features**:
- Automatic retry logic (max 3 per agent)
- Intelligent rollback to previous phases
- Overall confidence calculation
- Real-time progress tracking
- Error recovery and handling

## 🎨 Design System

Glassflow features a custom design system with:
- Monochrome palette with strategic glass-blue accents
- 8px grid system
- Consistent spacing and typography
- Glass morphism effects
- Accessible components

See [docs/design-system/](./docs/design-system/) for details.

## 📚 Documentation

- **System**: [AGENT_STATUS.md](./AGENT_STATUS.md) - Complete system overview
- **Security**: [SECURITY.md](./SECURITY.md) - API key management
- **Testing**: [TESTING_AGENTS.md](./TESTING_AGENTS.md) - How to test agents
- **Agents**:
  - [Architect Agent](./docs/agents/ARCHITECT_AGENT.md)
  - [Engineer Agent](./docs/archive/phase-3.md)
  - [Quality Agent](./docs/agents/QUALITY_AGENT.md)
  - [Debug Agent](./docs/agents/DEBUG_AGENT.md)
- **Orchestrator**: [docs/ORCHESTRATOR.md](./docs/ORCHESTRATOR.md)
- **Design System**: [docs/design-system/](./docs/design-system/)

## 🧪 Testing

```bash
# Validate entire system
bash scripts/validate-all.sh

# Validate individual agents
bash scripts/validate-architect.sh
bash scripts/validate-engineer.sh
bash scripts/validate-quality.sh

# Run unit tests
npm test

# Run E2E tests
npx playwright test
```

### Testing Agents via UI

1. Run `npm run tauri dev`
2. Select agent tab (Pipeline, Architect, Engineer, Quality, or Debug)
3. Enter input
4. Click "Test Agent" or "Run Full Pipeline"
5. View results

## 🛠️ Development

### Prerequisites
- Node.js 18+
- Rust 1.70+
- Tauri CLI

### Environment Variables

**Required**: Create a `.env` file with your OpenRouter API key:

```bash
OPENROUTER_API_KEY=sk-or-v1-your-actual-key-here
OPENROUTER_MODEL=x-ai/grok-4-fast:free
OPENROUTER_BASE_URL=https://openrouter.ai/api/v1
```

Get your API key from: https://openrouter.ai/keys

⚠️ **Security**: Never commit `.env` to git! It's already in `.gitignore`.

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

## 🎊 System Status

**✅ PRODUCTION READY**

- **Agents**: 4/4 complete (Architect, Engineer, Quality, Debug)
- **Orchestrator**: ✅ Complete
- **IPC Commands**: 25 registered
- **Documentation**: ✅ Complete
- **Security**: ✅ API keys protected
- **UI**: ✅ Full pipeline testing interface

## 📊 Statistics

- **Total Lines**: ~15,000+ lines of Rust + TypeScript
- **Modules**: 24 agent modules + 3 orchestrator modules
- **Test Coverage**: Unit tests + E2E tests
- **Validation Scripts**: 4 scripts

---

**Last Updated**: October 2025  
**Version**: 0.1.0  
**Status**: ✅ All systems operational
