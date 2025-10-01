# Troubleshooting Guide

## Quick Fixes

### 🔴 "Rollback required" Error

**What you see**:
```
❌ Pipeline Failed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Error: Pipeline failed at Building: [actual error]. Rollback not implemented.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**What it means**: An error occurred during code generation and the system tried to rollback (which isn't implemented yet).

**How to fix**:
1. Look at the actual error message in the output
2. Most common cause: Missing or invalid API key
3. Follow the specific error guidance below

---

### 🔑 API Key Issues

#### "OPENROUTER_API_KEY environment variable not set"

**Fix**:
```bash
# 1. Copy the example file
cp .env.example .env

# 2. Edit .env and add your key
# Get key from: https://openrouter.ai/keys
OPENROUTER_API_KEY=sk-or-v1-your-key-here

# 3. Restart the app
npm run tauri dev
```

#### "Invalid OpenRouter API key format"

**Fix**: Your API key should start with `sk-or-v1-`. Get a new key from https://openrouter.ai/keys

#### "LLM API error (401): Unauthorized"

**Fix**: Your API key is invalid or expired. Get a new one from https://openrouter.ai/keys

#### "LLM API error (429): Too Many Requests"

**Fix**: You've hit the rate limit or run out of credits. Check https://openrouter.ai/ for your usage

---

### ⏱️ Timeout Errors

**What you see**:
```
⏱️ Operation timed out. The LLM may be taking longer than expected.

💡 Suggestions:
  • Try a simpler prompt
  • The LLM may be experiencing high load
  • Try again in a few moments
```

**How to fix**:
1. Use a simpler, more specific prompt
2. Wait a few minutes and try again
3. Check OpenRouter status: https://status.openrouter.ai/

---

### 📝 Parse/JSON Errors

**What you see**:
```
📝 Failed to parse LLM response. The model may have returned invalid data.

💡 Suggestions:
  • The LLM returned invalid data
  • Try rephrasing your prompt to be more specific
  • This is usually a temporary issue - try again
```

**How to fix**:
1. Try again (often works on second try)
2. Make your prompt more specific
3. Try a different model in `.env`:
   ```
   OPENROUTER_MODEL=anthropic/claude-3.5-sonnet
   ```

---

### 🌐 Network Errors

**What you see**:
```
Failed to send request: [network error]
```

**How to fix**:
1. Check your internet connection
2. Check if you can access https://openrouter.ai/
3. Try disabling VPN if using one
4. Check firewall settings

---

## Step-by-Step Debugging

### 1. Check Environment Setup

```bash
# Verify .env file exists
ls -la .env

# Check if API key is set (should show key)
cat .env | grep OPENROUTER_API_KEY

# Verify key format (should start with sk-or-v1-)
```

### 2. Check Terminal Output

Look for these messages in the terminal running `npm run tauri dev`:
- `❌ Failed to create LLM client` - API key issue
- `[Orchestrator] ❌ Error in` - Shows which phase failed
- `💡 This appears to be` - Helpful hints about error type

### 3. Check Browser Console

Open DevTools (F12) and look for:
- Red error messages
- `Pipeline error:` log entries
- Network tab for API call failures

### 4. Test API Key Manually

```bash
# Test your API key with curl
curl https://openrouter.ai/api/v1/models \
  -H "Authorization: Bearer YOUR_API_KEY_HERE"

# Should return list of models if key is valid
```

---

## Error Message Reference

### Frontend Errors (in UI)

| Error Message | Cause | Solution |
|--------------|-------|----------|
| "Pipeline Failed" | General error | Check specific error details |
| "Check that your OPENROUTER_API_KEY is set" | Missing .env | Create .env file |
| "Verify your API key is valid" | Invalid key | Get new key |
| "Check your internet connection" | Network issue | Check connectivity |
| "Try a simpler prompt" | Timeout | Simplify request |
| "The LLM returned invalid data" | Parse error | Retry or rephrase |

### Backend Errors (in terminal)

| Error Message | Cause | Solution |
|--------------|-------|----------|
| "Failed to create LLM client" | API key not set | Add to .env |
| "Invalid OpenRouter API key format" | Wrong key format | Check key format |
| "Error in Planning" | Architect failed | Check prompt clarity |
| "Error in Building" | Engineer failed | Check architecture plan |
| "Error in Validating" | Quality check failed | Check generated code |
| "Error in Testing" | Debug failed | Check code validity |

---

## Still Having Issues?

### 1. Enable Verbose Logging

Check terminal output for detailed error messages. All errors are logged with context.

### 2. Check System Requirements

- Node.js 18+ installed
- Rust toolchain installed
- Internet connection active
- Valid OpenRouter API key

### 3. Try a Simple Test

Use this minimal prompt to test:
```
Create a simple React component that displays "Hello World"
```

If this works, your setup is correct and the issue is with more complex prompts.

### 4. Reset Everything

```bash
# Stop the app (Ctrl+C)

# Clean build
cd src-tauri
cargo clean
cd ..

# Restart
npm run tauri dev
```

---

## Common Workflow Issues

### "Nothing happens when I click Run Pipeline"

**Check**:
1. Is the button disabled? (gray)
2. Did you enter a prompt?
3. Check browser console for errors

### "Pipeline starts but immediately fails"

**Most likely**: API key issue. Check `.env` file.

### "Pipeline runs but no code appears"

**Current behavior**: Mock files are shown. Real file generation coming soon.

### "Error boundary shows up"

**Cause**: React component crashed

**Fix**: Reload page. If persists, check browser console.

---

## Getting Help

If you're still stuck:

1. **Check the logs**: Terminal + Browser Console
2. **Note the error message**: Copy the full error
3. **Check your setup**: API key, .env file, internet
4. **Try the simple test**: Use minimal prompt
5. **Check OpenRouter**: https://openrouter.ai/ for service status

## Prevention Tips

✅ **Always check .env file** before starting
✅ **Use specific prompts** instead of vague ones
✅ **Monitor API usage** at https://openrouter.ai/
✅ **Keep terminal visible** to see backend errors
✅ **Check browser console** for frontend errors
