// T059: Engineer Agent Documentation

# Engineer Agent Documentation

## Overview

The Engineer Agent is responsible for transforming architecture plans into production-ready code. It generates files, validates quality, and ensures adherence to the architecture specification.

## Architecture

### Core Components

1. **Engineer** (`src-tauri/src/agents/engineer.rs`)
   - Main agent orchestrator
   - State machine management
   - Workflow coordination

2. **PlanAnalyzer** (`src-tauri/src/agents/engineer/plan_analysis.rs`)
   - Parses architecture plans
   - Builds dependency graphs
   - Determines generation order

3. **CodeGenerator** (`src-tauri/src/agents/engineer/code_generator.rs`)
   - LLM-based code generation
   - Template-driven prompts
   - Import/export parsing

4. **QualityChecker** (`src-tauri/src/agents/engineer/quality_checker.rs`)
   - Syntax validation
   - Type safety checks
   - Import resolution
   - Style compliance

5. **AutoFixer** (`src-tauri/src/agents/engineer/auto_fixer.rs`)
   - Automatic issue resolution
   - Low/Medium severity fixes
   - Code formatting

6. **ConfidenceCalculator** (`src-tauri/src/agents/engineer/confidence.rs`)
   - Quality scoring
   - Plan adherence measurement
   - Issue penalty calculation

## State Machine

```
IDLE → ANALYZING_PLAN → GENERATING_CODE → REVIEWING → COMPLETE
                ↓              ↓              ↓
              ERROR ← ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

### States

- **Idle**: Waiting for architecture plan
- **AnalyzingPlan**: Validating plan and building dependency graph
- **GeneratingCode**: Creating files (with concurrent generation)
- **Reviewing**: Quality checks and auto-fixes
- **Complete**: All files generated successfully
- **Error**: Generation failed (recoverable)

## Usage

### Rust API

```rust
use glassflow::agents::engineer::Engineer;
use glassflow::models::EngineerArchitecturePlan;

let llm_client = create_llm_client();
let mut engineer = Engineer::new(llm_client);

let output = engineer.implement_plan(plan).await?;

println!("Generated {} files", output.files.len());
println!("Confidence: {:.1}%", output.confidence.overall);
```

### IPC Commands

```typescript
import { invoke } from '@tauri-apps/api/core';

// Start generation
await invoke('start_code_generation', {
  plan: architecturePlan,
  config: {
    outputDirectory: './output',
    maxConcurrent: 3,
    timeoutSeconds: 120,
  },
});

// Get state
const state = await invoke('get_engineer_state');

// Get progress
const progress = await invoke('get_generation_progress');

// Cancel
await invoke('cancel_generation');

// Export
await invoke('export_generated_code', {
  directory: './export',
});
```

### React Components

```tsx
import { EngineerPanel } from './components/engineer/EngineerPanel';

<EngineerPanel
  architecturePlan={plan}
  onComplete={(output) => {
    console.log('Generation complete!', output);
  }}
/>
```

## Quality Checks

The Engineer Agent performs comprehensive quality checks:

1. **Syntax Validation**: TypeScript compiler integration
2. **Type Safety**: No 'any' types, explicit return types
3. **Import Resolution**: All imports resolve correctly
4. **Export Validation**: Proper module exports
5. **Style Compliance**: ESLint rules, line length
6. **Edge Cases**: Error handling, null safety
7. **Documentation**: JSDoc comments

## Confidence Calculation

```
Overall = (Quality × 0.50) + (Adherence × 0.30) + ((100 - Penalty) × 0.20)
```

- **Quality Score**: Based on passed checks (0-100%)
- **Plan Adherence**: File count, line count, dependencies (0-100%)
- **Issue Penalty**: Severity-weighted (Critical: 20%, High: 10%, Medium: 5%, Low: 2%)

## Concurrent Generation

Files are generated in parallel when possible:

- **Independent files**: Generated concurrently (max 3-5)
- **Dependent files**: Generated in dependency order
- **Timeout handling**: 2-minute timeout per file with user prompt

## Error Handling

- **Recoverable errors**: Preserve completed files, allow retry
- **Critical errors**: Transition to ERROR state
- **Timeout**: Prompt user (Continue/Cancel/Skip)
- **Partial generation**: Only completed files are written

## Testing

### Unit Tests

```bash
cargo test --package glassflow --lib agents::engineer
```

### Integration Tests

```bash
cargo test --package glassflow --test full_generation_flow_test
cargo test --package glassflow --test self_review_cycle_test
cargo test --package glassflow --test error_recovery_test
```

### Contract Tests

```bash
cargo test --package glassflow --test engineer_commands_test
cargo test --package glassflow --test engineer_events_test
```

## Performance

- **Average generation time**: 30-60 seconds for 10-20 files
- **Concurrent limit**: 3-5 files simultaneously
- **Memory usage**: ~200MB per concurrent file
- **LLM calls**: 1 per file + 1 for quality review

## Configuration

```rust
pub struct GenerationConfig {
    pub output_directory: String,
    pub max_concurrent: usize,      // Default: 3
    pub timeout_seconds: u64,       // Default: 120
}
```

## Events

The Engineer Agent emits the following events:

- `engineer:state_changed`: State transitions
- `engineer:file_started`: File generation begins
- `engineer:file_completed`: File generation completes
- `engineer:reasoning`: Decision explanations
- `engineer:progress`: Progress updates
- `engineer:error`: Error occurrences
- `engineer:timeout_prompt`: Timeout user prompt
- `engineer:complete`: Generation complete

## Best Practices

1. **Validate plans**: Always validate architecture plans before generation
2. **Monitor progress**: Listen to progress events for UI updates
3. **Handle timeouts**: Implement timeout prompt UI
4. **Preserve work**: On error, save completed files
5. **Review quality**: Check confidence scores before accepting output
6. **Test output**: Run tests on generated code before deployment

## Troubleshoads

### Generation fails immediately
- Check architecture plan validity
- Verify LLM API credentials
- Ensure output directory is writable

### Timeout on every file
- Increase timeout_seconds
- Check LLM API response times
- Reduce file complexity

### Low confidence scores
- Review quality issues in report
- Check plan adherence metrics
- Validate architecture plan completeness

### Import resolution errors
- Verify dependencies in architecture plan
- Check relative path correctness
- Ensure all referenced files are generated

## Future Enhancements

- [ ] Incremental generation (update existing files)
- [ ] Custom quality rules
- [ ] Multi-language support (Python, Rust, etc.)
- [ ] Integration with external linters
- [ ] Code coverage analysis
- [ ] Performance profiling
