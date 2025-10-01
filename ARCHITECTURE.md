# Glassflow - Architecture

## Component Hierarchy

```
App
├── Header
│   ├── Logo
│   │   ├── SVG Icon (Glass Prism)
│   │   └── Text (Gradient)
│   └── Status Indicator
│
├── CommandInput
│   └── GlassSurface
│       └── Input Field
│
├── Welcome Screen (conditional)
│   └── GlassSurface
│       ├── Title (shimmer effect)
│       └── Feature List
│
├── AgentOrchestrator (conditional)
│   ├── Phase Announcement
│   └── Agent Grid
│       ├── AgentCard (Planner)
│       │   └── GlassSurface
│       │       ├── Name & Status
│       │       └── Reasoning Steps
│       ├── AgentCard (Executor)
│       │   └── GlassSurface
│       └── AgentCard (Validator)
│           └── GlassSurface
│
└── Command History (conditional)
    └── History Items
        └── GlassSurface (for each)
```

## Data Flow

```
User Input
    ↓
CommandInput
    ↓
handleCommandSubmit()
    ↓
Update State
    ├── currentCommand
    ├── commandHistory
    └── showWelcome
    ↓
AgentOrchestrator
    ↓
simulateOrchestration()
    ├── Phase 1: Planner (thinking)
    ├── Phase 2: Executor (executing)
    └── Phase 3: Validator (validating)
    ↓
handleOrchestrationComplete()
    ↓
Update History Status
    ↓
Show History
```

## State Management

### App State
```typescript
{
  currentCommand: string | null,
  commandHistory: CommandHistory[],
  showWelcome: boolean
}
```

### Agent State
```typescript
{
  agents: Agent[],
  orchestrationPhase: string
}
```

### Agent Type
```typescript
interface Agent {
  id: string,
  name: string,
  status: "idle" | "thinking" | "executing" | "complete",
  reasoning: string[]
}
```

## Component Props

### GlassSurface
```typescript
{
  children: ReactNode,
  className?: string,
  glow?: boolean,
  glowColor?: "white" | "blue",
  shimmer?: boolean,
  pulse?: boolean,
  onClick?: () => void
}
```

### CommandInput
```typescript
{
  onSubmit: (command: string) => void,
  placeholder?: string
}
```

### AgentCard
```typescript
{
  name: string,
  status: "idle" | "thinking" | "executing" | "complete",
  reasoning?: string[],
  delay?: number
}
```

### AgentOrchestrator
```typescript
{
  command: string,
  onComplete?: () => void
}
```

## Animation Timeline

### Welcome Screen
```
0ms:    Fade in + slide up
400ms:  Title shimmer starts
500ms:  Feature list appears
```

### Command Input
```
Focus:
  - Glow effect (blue)
  - Shimmer border
  - Scale 1.01
```

### Agent Orchestration
```
0ms:     Phase announcement
1000ms:  Planner appears (thinking)
3000ms:  Planner completes
3500ms:  Executor starts (executing)
6000ms:  Executor completes
6500ms:  Validator starts (thinking)
8500ms:  Validator completes
9500ms:  Show history
```

### Agent Card States
```
idle:      Gray text, no effects
thinking:  Blue text, pulse, glow
executing: Bright blue, glow
complete:  White text, no effects
```

## Styling System

### Colors (Tailwind)
```css
black:           #000000
white:           #FFFFFF
gray-300:        #d1d5db
gray-400:        #9ca3af
gray-500:        #6b7280
glass-blue-400:  #3385ff
glass-blue-500:  #0066ff
```

### Effects
```css
glass-effect:
  - background: rgba(255, 255, 255, 0.05)
  - backdrop-filter: blur(10px)
  - border: 1px solid rgba(255, 255, 255, 0.1)

glass-blue-glow:
  - box-shadow: 0 0 20px rgba(0, 102, 255, 0.3)
  - box-shadow: 0 0 40px rgba(0, 102, 255, 0.2)
  - box-shadow: inset 0 0 20px rgba(0, 102, 255, 0.1)

text-shimmer:
  - linear-gradient(90deg, ...)
  - animation: shimmer 3s linear infinite
```

## Backend Architecture

### Tauri Commands
```rust
process_agent_task(task: AgentTask) -> Result<AgentResponse>
get_available_agents() -> Vec<String>
```

### Data Structures
```rust
struct AgentTask {
    id: String,
    agent_type: String,
    description: String,
    status: String,
    reasoning: Vec<String>,
}

struct AgentResponse {
    task_id: String,
    result: String,
    reasoning_steps: Vec<String>,
}
```

## Future Integration Points

### 1. Real AI Backend
```
Frontend → Tauri Command → AI Service → Response
```

### 2. Streaming Responses
```
Frontend ← WebSocket ← Tauri ← AI Service
```

### 3. Persistence
```
Frontend → Tauri → SQLite/File System
```

### 4. Settings
```
Frontend ↔ Tauri ↔ Config File
```

## Performance Considerations

### Optimizations
- Framer Motion uses GPU acceleration
- Tailwind purges unused CSS
- React.memo for expensive components
- Lazy loading for future features

### Bundle Size
- Current: ~266KB (gzipped: ~86KB)
- Target: <500KB for production

## Security

### Current
- Tauri CSP enabled
- No external API calls
- Local-only execution

### Future
- API key management
- Secure storage for credentials
- Rate limiting
- Input validation

## Testing Strategy

### Unit Tests (Future)
- Component rendering
- State management
- Utility functions

### Integration Tests (Future)
- Command flow
- Agent orchestration
- Backend communication

### E2E Tests (Future)
- Full user workflows
- Cross-platform testing
- Performance benchmarks
