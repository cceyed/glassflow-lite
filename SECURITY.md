# Security Guidelines

## API Key Management

### ⚠️ CRITICAL: Never Commit API Keys

This project uses OpenRouter API keys which **MUST NEVER** be committed to git.

### Setup Instructions

1. **Copy the example file:**
   ```bash
   cp .env.example .env
   ```

2. **Add your API key to `.env`:**
   ```env
   OPENROUTER_API_KEY=sk-or-v1-your-actual-key-here
   OPENROUTER_MODEL=x-ai/grok-4-fast:free
   OPENROUTER_BASE_URL=https://openrouter.ai/api/v1
   ```

3. **Verify `.env` is in `.gitignore`:**
   The `.env` file is already added to `.gitignore` and will not be committed.

### Getting an API Key

1. Visit [OpenRouter](https://openrouter.ai/keys)
2. Sign up or log in
3. Generate a new API key
4. Copy the key (starts with `sk-or-v1-`)
5. Add it to your `.env` file

### Current Configuration

- **Provider**: OpenRouter
- **Model**: x-ai/grok-4-fast:free (free tier)
- **Future**: Will migrate to Anthropic Claude API

### Security Checklist

- ✅ `.env` is in `.gitignore`
- ✅ `.env.example` contains no real keys
- ✅ API key validation in LLM client
- ✅ No hardcoded keys in source code
- ✅ Environment variables loaded at runtime

### What to Do If You Accidentally Commit a Key

1. **Immediately revoke the key** at [OpenRouter Keys](https://openrouter.ai/keys)
2. Generate a new key
3. Update your `.env` file
4. Remove the key from git history:
   ```bash
   git filter-branch --force --index-filter \
     "git rm --cached --ignore-unmatch .env" \
     --prune-empty --tag-name-filter cat -- --all
   ```
5. Force push (if necessary and safe to do so)

### Best Practices

- Never log API keys
- Never echo API keys in terminal
- Never share `.env` files
- Use different keys for development and production
- Rotate keys regularly
- Monitor API usage for anomalies

## Agent Security

### LLM Client Security

- API keys are validated on initialization
- Keys must start with `sk-or-v1-`
- Failed validation prevents agent startup
- Clear error messages without exposing keys

### Data Privacy

- User specifications are sent to OpenRouter/Grok
- Generated code remains local
- No telemetry or tracking
- All data processing happens locally except LLM calls

## Reporting Security Issues

If you discover a security vulnerability, please email the maintainers directly. Do not open a public issue.
