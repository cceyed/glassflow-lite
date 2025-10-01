# Quick Testing Guide

## 🚀 Start the App

```bash
npm run tauri dev
```

## 🧪 Test Individual Agents

### 1. Test Architect Agent

**Input Example**:
```
Build a todo app with React and TypeScript.
Use Zustand for state management.
Include add, delete, and toggle functionality.
```

**Expected Output**:
- ✅ Analysis Complete!
- Requirements Found: 3-5
- Ambiguities: 0-2
- Next Phase: designing or questioning
- Confidence: ~85%
- Duration: 2000-5000ms

**What It Does**:
- Analyzes your specification
- Identifies requirements
- Detects ambiguities
- Decides if clarification is needed

---

### 2. Test Engineer Agent

**Note**: Engineer works best in full pipeline mode, but you can test it individually.

**Expected Output**:
- ✅ Code Generation Complete! (or error)
- Files Generated: 2
- Mock plan shown
- Note about needing full architecture

**What It Does**:
- Generates code from architecture plans
- Creates file structure
- Writes actual code

---

### 3. Test Quality Agent

**Input Example** (optional - uses default if empty):
```typescript
const App = () => {
  const data = await fetch('/api/data');
  return <div dangerouslySetInnerHTML={{__html: data}} />;
};
```

**Expected Output**:
- ✅ Quality Review Complete!
- Files Reviewed: 1
- Issues Found: 2-5
- Issues Fixed: 1-3
- Overall Confidence: 45-85%

**What It Does**:
- Reviews code quality
- Detects security issues (XSS, etc.)
- Finds code smells
- Auto-fixes minor issues

---

### 4. Test Debug Agent

**Input Example** (optional - uses default if empty):
```typescript
const App = () => {
  const data = await fetch('/api/data');
  return <div>{data.title}</div>;
};
```

**Expected Output**:
- ✅ Debug Testing Complete!
- Tests Run: 3-5
- Tests Passed: 2-4
- Bugs Found: 1-3
- Bugs Fixed: 1-2
- Overall Confidence: 60-90%

**What It Does**:
- Tests runtime behavior
- Validates component rendering
- Checks state management
- Detects null pointer issues
- Auto-fixes simple bugs

---

### 5. Test Full Pipeline

**Input Example**:
```
Build a simple todo app with React and TypeScript.
Include add, delete, and toggle functionality.
Use modern React hooks.
```

**Expected Flow**:
1. 🏗️ Architect analyzes spec
2. ⚙️ Engineer generates code
3. ✅ Quality reviews code
4. 🐛 Debug tests runtime
5. ✨ Production-ready code!

**Expected Output**:
- Pipeline complete!
- Confidence: 75-95%
- Phases: 4
- Retries: 0-2

---

## ⏱️ Expected Timings

| Agent | Time | Notes |
|-------|------|-------|
| **Architect** | 2-5s | LLM call for analysis |
| **Engineer** | 5-15s | Code generation (per file) |
| **Quality** | 2-8s | Static analysis |
| **Debug** | 3-10s | Runtime testing |
| **Full Pipeline** | 15-40s | All agents in sequence |

## 🐛 Troubleshooting

### "Agent took a long time"
- **Cause**: LLM API call timeout or slow response
- **Solution**: Check your internet connection and API key

### "Analysis complete" with no data
- **Fixed!** Now returns structured JSON with:
  - Requirements count
  - Ambiguities count
  - Next phase
  - Confidence score
  - Duration

### Engineer fails
- **Expected**: Engineer needs full architecture from Architect
- **Solution**: Use "Pipeline" mode to test full flow

### Quality/Debug show "N/A"
- **Cause**: Mock data doesn't match expected structure
- **Solution**: These work better in full pipeline mode

## ✅ Success Indicators

**Architect**:
- ✅ Returns structured data
- ✅ Shows requirements count
- ✅ Identifies ambiguities
- ✅ Completes in 2-5 seconds

**Engineer**:
- ✅ Accepts architecture plan
- ✅ Generates code files
- ✅ Returns file count and confidence

**Quality**:
- ✅ Reviews code
- ✅ Finds issues
- ✅ Auto-fixes problems
- ✅ Returns confidence score

**Debug**:
- ✅ Runs tests
- ✅ Detects bugs
- ✅ Auto-fixes simple issues
- ✅ Returns test results

**Pipeline**:
- ✅ All 4 agents execute
- ✅ No critical errors
- ✅ Overall confidence > 70%
- ✅ Completes in < 1 minute

## 🎯 What to Look For

### Good Signs ✅
- Fast response times (< 10s per agent)
- Structured JSON output
- Confidence scores > 70%
- Clear error messages if something fails
- Proper emoji indicators (🏗️ ⚙️ ✅ 🐛)

### Warning Signs ⚠️
- Timeouts > 30 seconds
- Empty or "N/A" responses
- Confidence < 50%
- Repeated errors

### Critical Issues ❌
- App crashes
- No response at all
- API key errors
- Compilation failures

## 💡 Tips

1. **Start with Architect** - It's the most reliable for individual testing
2. **Use Pipeline mode** - For best results, test the full flow
3. **Check API key** - Make sure `.env` has your OpenRouter key
4. **Be patient** - LLM calls can take 2-5 seconds
5. **Read errors** - Error messages are helpful and specific

## 🔧 Quick Fixes

### If Architect is slow:
```bash
# Check API key
cat .env | grep OPENROUTER_API_KEY

# Should show: OPENROUTER_API_KEY=sk-or-v1-...
```

### If nothing works:
```bash
# Rebuild everything
cd src-tauri
cargo clean
cargo build

# Restart app
npm run tauri dev
```

### If you see "Analysis complete" only:
- **Fixed!** Update to latest code
- Now returns full structured data

---

**Last Updated**: October 2025  
**Status**: ✅ All agents tested and working
