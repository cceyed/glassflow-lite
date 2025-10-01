# Error Handling & Boundaries

## Overview
Comprehensive error handling has been implemented across the application to provide clear feedback and graceful degradation.

## React Error Boundary

### Location
`src/components/ErrorBoundary.tsx`

### Features
- Catches React component errors
- Displays user-friendly error UI
- Shows error message and stack trace
- Provides "Try Again" and "Reload Page" buttons
- Wraps the entire application in `App.tsx`

### Usage
```tsx
<ErrorBoundary>
  <YourComponent />
</ErrorBoundary>
```

## Pipeline Error Handling

### Frontend (PipelineUI.tsx)
Enhanced error handling with:
- Detailed error messages
- Context-specific suggestions based on error type
- Visual error state indicator
- Console logging for debugging

#### Error Types Detected
1. **API/LLM Errors**
   - Suggests checking API key
   - Verifies API credits
   - Checks internet connection

2. **Timeout Errors**
   - Suggests simpler prompts
   - Notes high load possibility
   - Recommends retry

3. **Parse/JSON Errors**
   - Indicates invalid LLM response
   - Suggests rephrasing prompt
   - Notes temporary nature

4. **General Errors**
   - Console debugging tips
   - Configuration checks
   - Alternative prompts

### Backend (Orchestrator)
Enhanced error handling in `src-tauri/src/orchestrator/mod.rs`:

#### Features
- Detailed error context
- Phase-specific error messages
- Helpful diagnostic hints
- Improved error propagation

#### Error Decision Logic
1. **Critical/Fatal Errors** → Abort immediately
2. **Clarification Needed** → Rollback to planning (not yet implemented)
3. **Retriable Errors** → Retry up to 3 times per phase
4. **Max Retries Exceeded** → Abort with context

#### Error Messages Include
- Phase where error occurred
- Original error message
- Helpful context (API, timeout, parse, etc.)
- Suggestions for resolution
- Retry attempt information

## Common Error Scenarios

### 1. Missing API Key
**Error**: "LLM API error" or "API key not found"

**Solution**:
1. Create `.env` file in project root
2. Add: `ANTHROPIC_API_KEY=your_key_here`
3. Restart the application

### 2. Invalid API Response
**Error**: "Failed to parse LLM response" or "JSON parse error"

**Cause**: LLM returned malformed data

**Solution**:
- Usually temporary - retry
- Try more specific prompts
- Check LLM service status

### 3. Timeout
**Error**: "Operation timed out"

**Cause**: LLM taking too long (>2 minutes per file)

**Solution**:
- Simplify the prompt
- Break into smaller tasks
- Check LLM service load

### 4. Rollback Required
**Error**: "Rollback required" or "Rollback not implemented"

**Cause**: Error recovery strategy triggered

**Current Behavior**: Aborts with detailed error message

**Future**: Will implement actual rollback to previous phase

## Error Logging

### Frontend
- All errors logged to browser console
- Visible in DevTools Console tab
- Includes full error object

### Backend
- Errors logged to stderr
- Visible in terminal running `npm run tauri dev`
- Includes stack traces

## Testing Error Handling

### Test React Error Boundary
```tsx
// Add to any component to trigger error
throw new Error('Test error boundary');
```

### Test Pipeline Errors
1. Use invalid API key
2. Use extremely complex prompt
3. Disconnect internet during run
4. Use malformed input

## Future Improvements

1. **Rollback Implementation**
   - Actually return to previous phase
   - Preserve partial results
   - Allow user to modify and retry

2. **Retry UI**
   - Show retry attempts in UI
   - Allow manual retry control
   - Display retry countdown

3. **Error Recovery**
   - Auto-save progress
   - Resume from last good state
   - Partial result preservation

4. **Telemetry**
   - Error rate tracking
   - Common error patterns
   - Performance metrics

5. **User Feedback**
   - Error reporting button
   - Automatic diagnostics
   - Suggested fixes based on history
