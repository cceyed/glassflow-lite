# Testing the Multi-Agent System

## Quick Start

### 1. Setup API Key

```bash
# Copy the example environment file
cp .env.example .env

# Edit .env and add your OpenRouter API key
# Your key should start with: sk-or-v1-
```

Your `.env` file should look like:
```env
OPENROUTER_API_KEY=sk-or-v1-your-actual-api-key-here
OPENROUTER_MODEL=x-ai/grok-4-fast:free
OPENROUTER_BASE_URL=https://openrouter.ai/api/v1

### 2. Run the Application

```bash
npm run tauri dev
```

### 3. Test Each Agent

The UI provides a comprehensive testing interface for all three agents:

## Agent Testing Guide

### 🏗️ Architect Agent

**Purpose**: Analyzes specifications and creates architecture plans

**Test Input Example**:
```
Build a todo app with React and TypeScript.
Use Zustand for state management and Tailwind for styling.
```

**Expected Output**:
- Specification analysis
- Architecture plan with file structure
- Technology stack decisions
- Confidence score

**IPC Commands**:
- `architect_analyze` - Start analysis
- `architect_get_state` - Get current state
- `architect_answer` - Answer clarification questions
- `architect_export_plan` - Export the plan

### ⚙️ Engineer Agent

**Purpose**: Generates code from architecture plans

**Test Input**: Auto-generated mock plan (no manual input needed)

**Expected Output**:
- Generated code files
- Quality checks
- Confidence scores
- Generation metadata

**IPC Commands**:
- `start_code_generation` - Start generation
- `get_engineer_state` - Get current state
- `get_generation_progress` - Get progress
- `cancel_generation` - Cancel generation
- `retry_generation` - Retry failed generation
- `get_quality_report` - Get quality report
- `export_generated_code` - Export code

### ✅ Quality Agent

**Purpose**: Reviews code quality and identifies issues

**Test Input Example**:
```typescript
const App = () => {
  const data = await fetch('/api/data');
  return <div dangerouslySetInnerHTML={{__html: data}} />;
};
```

**Expected Output**:
- Quality issues found
- Auto-fixes applied
- Confidence breakdown
- Security warnings

**IPC Commands** (to be implemented):
- `quality_review_code` - Start review
- `quality_get_report` - Get report
- `quality_apply_fixes` - Apply auto-fixes

## Validation Scripts

### Validate All Agents

```bash
# Architect
bash scripts/validate-architect.sh

# Engineer
bash scripts/validate-engineer.sh

# Quality
bash scripts/validate-quality.sh
```

## Testing Workflow

### Full Pipeline Test

1. **Architect**: Analyze a specification
   ```
   Input: "Build a React todo app"
   Output: Architecture plan
   ```

2. **Engineer**: Generate code from plan
   ```
   Input: Architecture plan from step 1
   Output: Generated code files
   ```

3. **Quality**: Review generated code
   ```
   Input: Code files from step 2
   Output: Quality report with fixes
   ```

### Individual Agent Tests

#### Test Architect Alone
```typescript
// In the UI, select "Architect"
// Enter: "Build a todo app with React"
// Click "Test Architect Agent"
// Observe: Specification analysis and plan
```

#### Test Engineer Alone
```typescript
// In the UI, select "Engineer"
// Click "Test Engineer Agent"
// Uses mock plan automatically
// Observe: Code generation process
```

#### Test Quality Alone
```typescript
// In the UI, select "Quality"
// Enter some code to review
// Click "Test Quality Agent"
// Observe: Quality issues and fixes
```

## Troubleshooting

### API Key Issues

**Problem**: "OPENROUTER_API_KEY not set"
**Solution**: 
1. Check `.env` file exists
2. Verify key starts with `sk-or-v1-`
3. Restart the application

### Agent Not Responding

**Problem**: Agent hangs or doesn't respond
**Solution**:
1. Check console for errors
2. Verify API key is valid
3. Check network connection
4. Try with a simpler input

### Compilation Errors

**Problem**: Rust compilation fails
**Solution**:
```bash
cd src-tauri
cargo clean
cargo build
```

### TypeScript Errors

**Problem**: TypeScript compilation fails
**Solution**:
```bash
npm install
npm run check
```

## API Usage

### OpenRouter Free Tier

- **Model**: x-ai/grok-4-fast:free
- **Rate Limits**: Check OpenRouter dashboard
- **Cost**: Free tier available
- **Monitoring**: View usage at https://openrouter.ai/activity

### Future Migration to Anthropic

The system is designed to easily switch to Anthropic Claude:

1. Update `.env`:
   ```env
   ANTHROPIC_API_KEY=sk-ant-...
   ANTHROPIC_MODEL=claude-3-5-sonnet-20241022
   ```

2. Update `llm/client.rs` to use Anthropic SDK

3. All agents will automatically use the new provider

## Performance Benchmarks

### Expected Timings

- **Architect Analysis**: 2-5 seconds
- **Engineer Generation**: 5-15 seconds (per file)
- **Quality Review**: 2-8 seconds

### Confidence Thresholds

- **95-100%**: Excellent, production-ready
- **85-94%**: Good, minor improvements possible
- **70-84%**: Acceptable, some concerns
- **Below 70%**: Needs review

## Next Steps

1. Test each agent individually
2. Test the full pipeline
3. Review generated code quality
4. Adjust confidence thresholds
5. Add custom quality rules
6. Integrate with your workflow

## Support

- Documentation: `docs/agents/`
- Security: `SECURITY.md`
- Architecture: `ARCHITECTURE.md`
- Issues: GitHub Issues
