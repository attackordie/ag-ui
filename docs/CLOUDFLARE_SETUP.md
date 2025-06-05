# Cloudflare API Token Setup for AG-UI Demo Deployment

This guide explains how to securely set up Cloudflare deployment for the AG-UI demo in a public repository.

## Overview

The AG-UI demo uses GitHub Actions to automatically deploy to Cloudflare Workers. To maintain security:
- API tokens are stored as GitHub Secrets (never in code)
- Tokens have minimal permissions (only what's needed)
- Tokens are scoped to specific resources

## Step 1: Create a Limited Cloudflare API Token

1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com/profile/api-tokens)
2. Click **"Create Token"**
3. Use **"Custom token"** template
4. Configure the token with these **minimal permissions**:

### Token Permissions:
```
Account:
- Cloudflare Workers Scripts:Edit

Zone:
- Workers Routes:Edit (if using custom domains)
```

### Token Resources:
```
Include:
- All accounts (or specific account if you prefer)
- Specific zone (optional, only if using custom domain)
```

### Token Options:
- **TTL**: Set an expiration date for extra security
- **IP Address Filtering**: Add your GitHub Actions IP ranges (optional)

5. Click **"Continue to summary"** → **"Create Token"**
6. **IMPORTANT**: Copy the token immediately (shown only once!)

## Step 2: Add Token to GitHub Secrets

1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **"New repository secret"**
4. Add:
   - **Name**: `CLOUDFLARE_API_TOKEN`
   - **Value**: Your token from Step 1
5. Click **"Add secret"**

## Step 3: Verify GitHub Actions Configuration

The workflow already references the secret correctly:

```yaml
- name: 🚀 Deploy to Cloudflare Workers
  env:
    CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
  run: |
    cd rust-sdk/ag-ui-wasm/examples/worker
    wrangler deploy --compatibility-date 2024-01-01
```

## Security Best Practices

### ✅ DO:
- Use minimal permissions (only Workers Scripts:Edit)
- Set token expiration dates
- Rotate tokens regularly
- Use GitHub Secrets for storage
- Monitor token usage in Cloudflare dashboard

### ❌ DON'T:
- Never commit tokens to code
- Never use account-level API keys
- Never share tokens publicly
- Never use tokens with unnecessary permissions

## Testing Your Setup

After configuration:
1. Push any change to the `main` branch
2. Check GitHub Actions logs
3. Verify deployment at your Workers URL

## Troubleshooting

### Error: "CLOUDFLARE_API_TOKEN environment variable not set"
- Ensure the secret name is exactly `CLOUDFLARE_API_TOKEN`
- Check that the secret was added to the correct repository

### Error: "Unauthorized"
- Verify token has `Cloudflare Workers Scripts:Edit` permission
- Check token hasn't expired
- Ensure token is for the correct Cloudflare account

### Error: "Workers.dev subdomain not configured"
- First deployment may need manual subdomain setup
- Visit Cloudflare Dashboard → Workers & Pages → Overview
- Set up your workers.dev subdomain

## Token Permissions Explained

**Why only "Workers Scripts:Edit"?**
- This is the minimal permission needed to deploy Workers
- It cannot access your DNS, SSL, or other account settings
- It cannot read sensitive account information
- It's scoped to only Workers functionality

**Additional Permissions (only if needed):**
- `Workers Routes:Edit` - Only if using custom domains
- `Zone:Read` - Only if the worker needs zone information

## Revoking Access

If token is compromised:
1. Go to [Cloudflare API Tokens](https://dash.cloudflare.com/profile/api-tokens)
2. Find the token and click **"Revoke"**
3. Create a new token with the same permissions
4. Update GitHub Secrets with the new token

## Resources

- [Cloudflare API Token Docs](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/)
- [GitHub Encrypted Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Wrangler Authentication](https://developers.cloudflare.com/workers/wrangler/authentication/)