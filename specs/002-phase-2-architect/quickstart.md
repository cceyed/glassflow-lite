# Quickstart: Architect Agent System

**Feature**: 002-phase-2-architect  
**Purpose**: Validate the Architect Agent implementation through end-to-end user scenarios

## Prerequisites

- Tauri app running (`npm run tauri dev`)
- LLM API key configured (OpenAI or Anthropic)
- Test fixtures available in `src-tauri/tests/fixtures/`

## Test Scenario 1: Happy Path (Clear Spec)

**User Story**: Developer submits clear specification, agent designs directly without questions

**Steps**:

1. **Launch Application**
   ```bash
   npm run tauri dev
   ```
   - ✅ App window opens
   - ✅ Architect Agent card shows "IDLE" state with ○ indicator

2. **Submit Clear Specification**
   - Input: "Build a React 18 + TypeScript todo app with Zustand state management and Tailwind CSS styling"
   - Click "Analyze" button
   - ✅ State transitions to "ANALYZING" with ◐ indicator
   - ✅ Reasoning panel shows: "Starting specification analysis"

3. **Observe Analysis**
   - ✅ Reasoning updates appear:
     - "Observation: User specified React 18"
     - "Observation: TypeScript specified"
     - "Analysis: Specification is complete"
   - ✅ State transitions to "DESIGNING" with ● indicator (skips QUESTIONING)
   - ✅ Progress bar appears (0-100%)

4. **Observe Design Progress**
   - ✅ Reasoning updates show:
     - "Designing tech stack"
     - "Planning file structure"
     - "Designing components"
     - "Making architecture decisions"
   - ✅ Progress bar advances
   - ✅ Elapsed time displays (<15s target)

5. **Review Complete Plan**
   - ✅ State transitions to "COMPLETE" with ✓ indicator
   - ✅ Confidence score displays (>85% expected)
   - ✅ Plan includes:
     - Project name: "todo-app"
     - Tech stack: React 18, TypeScript, Zustand, Tailwind
     - File structure with paths
     - Component hierarchy (App, TodoList, TodoItem, AddTodo)
     - Architecture decisions with reasoning
   - ✅ Auto-transitions to IDLE after 2 seconds

**Expected Duration**: <20 seconds total

---

## Test Scenario 2: Vague Spec with Questions

**User Story**: Developer submits vague specification, agent asks clarifying questions

**Steps**:

1. **Submit Vague Specification**
   - Input: "Build a todo app with React"
   - Click "Analyze"
   - ✅ State transitions to "ANALYZING"

2. **Observe Ambiguity Detection**
   - ✅ Reasoning shows:
     - "Analysis: Language not specified"
     - "Analysis: State management unclear"
     - "Analysis: Styling approach not specified"
   - ✅ State transitions to "QUESTIONING" with ◑ indicator

3. **Answer Question 1**
   - ✅ Question displays: "Which state management would you prefer?"
   - ✅ Options shown:
     - a) useState/useReducer (React built-in)
     - b) Redux Toolkit (robust, popular)
     - c) Zustand (lightweight) [Recommended]
     - d) Jotai (atomic)
   - ✅ Progress shows "Question 1 of 3"
   - Select: "c" or press Enter for default
   - ✅ Reasoning: "Received answer: Zustand"

4. **Answer Question 2**
   - ✅ Question: "Would you like to use TypeScript?"
   - ✅ Options: (Y/n) [Y is default]
   - Press Enter
   - ✅ Reasoning: "Received answer: Yes"

5. **Answer Question 3**
   - ✅ Question: "How should styling be handled?"
   - ✅ Options: Plain CSS, Tailwind, CSS Modules, Styled Components
   - Select: "Tailwind"
   - ✅ Reasoning: "All questions answered, designing architecture"

6. **Observe Design & Completion**
   - ✅ State transitions to "DESIGNING"
   - ✅ Progress bar shows design progress
   - ✅ State transitions to "COMPLETE"
   - ✅ Confidence score >90% (all ambiguities resolved)

**Expected Duration**: <30 seconds total (including user input time)

---

## Test Scenario 3: Error Handling (Invalid Spec)

**User Story**: Developer submits impossible specification, agent detects and errors gracefully

**Steps**:

1. **Submit Invalid Specification**
   - Input: "Build a React app that runs on MS-DOS"
   - Click "Analyze"
   - ✅ State transitions to "ANALYZING"

2. **Observe Error Detection**
   - ✅ Reasoning shows:
     - "Analysis: React requires modern browser"
     - "Analysis: MS-DOS has no JavaScript runtime"
     - "Conclusion: Specification is impossible"
   - ✅ State transitions to "ERROR" with ✗ indicator

3. **Review Error Message**
   - ✅ Error displays: "Cannot proceed: React requires modern browser environment, incompatible with MS-DOS"
   - ✅ Recoverable: true
   - ✅ Suggestions shown:
     - "Modify spec to use modern OS (Windows/macOS/Linux)"
     - "Consider CLI tool instead of React app"
   - ✅ Actions available: Retry, Modify Spec, Cancel

4. **Retry with Modified Spec**
   - Click "Modify Spec"
   - Input: "Build a React app for Windows desktop"
   - Click "Retry"
   - ✅ State transitions to "ANALYZING"
   - ✅ Successfully proceeds to DESIGNING

**Expected Duration**: <10 seconds to error, <20 seconds to recovery

---

## Test Scenario 4: Interrupted Session Recovery

**User Story**: Developer's session is interrupted, agent recovers state

**Steps**:

1. **Start Analysis**
   - Input: "Build a todo app"
   - Click "Analyze"
   - ✅ State transitions to "QUESTIONING"
   - ✅ Question 1 displays

2. **Answer First Question**
   - Select answer
   - ✅ State persisted to `~/.glassflow/architect-state.json`

3. **Simulate Interruption**
   - Close application (Cmd+Q or kill process)
   - ✅ State file exists and contains current state

4. **Reopen Application**
   - Launch app again
   - ✅ Agent loads from persisted state
   - ✅ Shows "QUESTIONING" state
   - ✅ Displays Question 2 (not Question 1)
   - ✅ Previous answer retained

5. **Complete Session**
   - Answer remaining questions
   - ✅ Proceeds to DESIGNING and COMPLETE normally

**Expected Behavior**: Seamless resume without data loss

---

## Test Scenario 5: Performance Validation

**User Story**: Verify performance targets are met

**Steps**:

1. **Measure Analysis Time**
   - Submit spec: "Build a todo app with React"
   - Start timer
   - ✅ ANALYZING → QUESTIONING transition <5 seconds

2. **Measure Design Time**
   - Submit clear spec (skip questions)
   - Start timer at DESIGNING state
   - ✅ DESIGNING → COMPLETE transition <15 seconds

3. **Measure UI Responsiveness**
   - During DESIGNING phase:
   - ✅ Reasoning updates stream in real-time (<100ms latency)
   - ✅ Progress bar updates smoothly (60fps)
   - ✅ State transition animations complete in 0.3-0.5s
   - ✅ UI remains responsive (no freezing)

**Expected Performance**:
- Analysis: <5s
- Design: <15s
- UI: 60fps, <500ms transitions

---

## Test Scenario 6: Confidence Scoring Validation

**User Story**: Verify confidence scores accurately reflect spec quality

**Steps**:

1. **Test High Confidence (Clear Spec)**
   - Input: "Build a React 18 + TypeScript todo app with Zustand and Tailwind CSS"
   - ✅ Confidence >85%
   - ✅ Breakdown shows:
     - Spec Clarity: >90%
     - Feasibility: >95%
     - Soundness: >90%
     - Completeness: >85%
     - Risk: >90%

2. **Test Low Confidence (Vague Spec)**
   - Input: "Build a website"
   - ✅ Confidence <60%
   - ✅ Breakdown shows:
     - Spec Clarity: <50% (many ambiguities)
     - Feasibility: ~70% (generic tech stack)
     - Soundness: ~60% (minimal architecture)
     - Completeness: <50% (missing details)
     - Risk: ~70%
   - ✅ High-risk items flagged in plan

3. **Test Medium Confidence (Partial Spec)**
   - Input: "Build a React todo app with authentication"
   - Answer questions partially
   - ✅ Confidence 60-85%
   - ✅ Breakdown reflects resolved vs. unresolved items

**Expected Behavior**: Confidence formula matches spec quality

---

## Automated Test Validation

**Run Unit Tests**:
```bash
cd src-tauri
cargo test
```
✅ All state transition tests pass  
✅ Ambiguity detection tests pass  
✅ Question generation tests pass  
✅ Confidence calculation tests pass

**Run Integration Tests**:
```bash
cd src-tauri
cargo test --test integration
```
✅ Full pipeline tests pass (mocked LLM)  
✅ JSON schema validation passes  
✅ Error recovery tests pass

**Run Frontend Tests**:
```bash
npm run test
```
✅ Component rendering tests pass  
✅ State management tests pass  
✅ Event handling tests pass

**Run E2E Tests**:
```bash
npm run test:e2e
```
✅ Playwright scenarios pass  
✅ UI interactions validated  
✅ State persistence validated

---

## Success Criteria

**Functional**:
- ✅ All 6 agent states implemented and transition correctly
- ✅ Ambiguity detection identifies missing fields, vague terms, contradictions
- ✅ Question generation limits to 7, combines related, offers defaults
- ✅ Architecture plans include all required sections
- ✅ Confidence scoring matches formula
- ✅ Error handling graceful with recovery paths
- ✅ State persistence enables resume

**Performance**:
- ✅ Analysis <5s
- ✅ Design <15s
- ✅ UI 60fps, transitions <500ms

**Quality**:
- ✅ All unit tests pass
- ✅ All integration tests pass
- ✅ All E2E tests pass
- ✅ No console errors or warnings
- ✅ Glass morphism design applied correctly

---

## Troubleshooting

**Issue**: LLM API calls fail  
**Solution**: Check API key in config, verify network connection, check rate limits

**Issue**: State persistence fails  
**Solution**: Check `~/.glassflow/` directory permissions, verify JSON format

**Issue**: UI freezes during design  
**Solution**: Verify async LLM calls (not blocking), check Tokio runtime

**Issue**: Confidence scores incorrect  
**Solution**: Verify formula implementation, check ambiguity counting, validate sub-scores

**Issue**: Questions not displaying  
**Solution**: Check Tauri event emission, verify frontend event listeners, check question generation logic

---

## Next Steps

After quickstart validation passes:
1. Run `/tasks` command to generate implementation tasks
2. Execute tasks in TDD order (tests first, then implementation)
3. Validate against quickstart scenarios continuously
4. Deploy to production once all scenarios pass
