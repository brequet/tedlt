---
title: Environment Variables
description: Configure Jira credentials with environment variables
---

Environment variables are used to securely store your Jira credentials. tedlt never stores sensitive information in configuration files.

## Required Variables

### `JIRA_API_TOKEN`

Your Jira API token for authentication.

```bash
JIRA_API_TOKEN=your_api_token_here
```

**How to generate:**
1. Go to [https://id.atlassian.com/manage-profile/security/api-tokens](https://id.atlassian.com/manage-profile/security/api-tokens)
2. Click **Create API token**
3. Give it a label (e.g., "tedlt CLI")
4. Click **Create** and copy the token

:::caution
API tokens are like passwords—keep them secure! Never commit them to version control.
:::

### `JIRA_EMAIL`

Your Jira account email address.

```bash
JIRA_EMAIL=your.email@example.com
```

This must match the email associated with your Jira account.

## Setting Environment Variables

You have two options for providing environment variables to tedlt.

### Option 1: `.env` File (Recommended)

Create a `.env` file in the directory where you run tedlt:

```bash
# .env
JIRA_API_TOKEN=your_api_token_here
JIRA_EMAIL=your.email@example.com
```

tedlt automatically loads this file when you run commands from that directory.

**Advantages:**
- No need to set variables in every shell session
- Easy to manage per-project credentials
- Works across different terminals and sessions
- Simple to share setup (without sharing actual credentials)

**Security:**
- Add `.env` to your `.gitignore` to prevent committing it
- Create `.env.example` with dummy values for documentation

```bash
# .env.example
JIRA_API_TOKEN=your_api_token_here
JIRA_EMAIL=your.email@example.com
```

### Option 2: Shell Environment Variables

Set environment variables directly in your shell.

#### Linux / macOS

**Bash / Zsh:**

```bash
export JIRA_API_TOKEN="your_api_token_here"
export JIRA_EMAIL="your.email@example.com"
```

To make them permanent, add to your shell configuration file:

```bash
# ~/.bashrc or ~/.zshrc
export JIRA_API_TOKEN="your_api_token_here"
export JIRA_EMAIL="your.email@example.com"
```

Then reload your shell:

```bash
source ~/.bashrc  # or ~/.zshrc
```

#### Windows

**PowerShell:**

```powershell
$env:JIRA_API_TOKEN="your_api_token_here"
$env:JIRA_EMAIL="your.email@example.com"
```

To make them permanent:

```powershell
[System.Environment]::SetEnvironmentVariable('JIRA_API_TOKEN', 'your_api_token_here', 'User')
[System.Environment]::SetEnvironmentVariable('JIRA_EMAIL', 'your.email@example.com', 'User')
```

**Command Prompt:**

```cmd
set JIRA_API_TOKEN=your_api_token_here
set JIRA_EMAIL=your.email@example.com
```

To make them permanent, use System Properties:
1. Search for "Environment Variables" in Windows settings
2. Click "Edit the system environment variables"
3. Click "Environment Variables"
4. Under "User variables", click "New"
5. Add `JIRA_API_TOKEN` and `JIRA_EMAIL`

## How tedlt Loads Environment Variables

tedlt looks for environment variables in this order:

1. **Current shell environment** - Variables already set in your terminal
2. **`.env` file in current directory** - Loaded automatically if it exists
3. **`.env` file in parent directories** - Searches up the directory tree

This means you can have different `.env` files for different projects:

```
~/work/
  project-a/
    .env          # Work Jira credentials
  project-b/
    .env          # Work Jira credentials

~/personal/
  my-project/
    .env          # Personal Jira credentials
```

## Multiple Jira Instances

If you work with multiple Jira instances, you can't use a single set of environment variables. Instead, use profiles:

### Approach 1: Multiple `.env` Files

Keep separate `.env` files in different directories:

```
~/work/
  .env          # Work credentials
~/personal/
  .env          # Personal credentials
```

Run tedlt from the appropriate directory.

### Approach 2: Switch Environment Variables

Create helper scripts to switch credentials:

```bash
# switch-to-work.sh
export JIRA_API_TOKEN="work_token"
export JIRA_EMAIL="work@company.com"

# switch-to-personal.sh
export JIRA_API_TOKEN="personal_token"
export JIRA_EMAIL="personal@email.com"
```

Source the appropriate script before using tedlt:

```bash
source switch-to-work.sh
tedlt create "Work task" --profile work
```

### Approach 3: Profile-Specific Credentials (Future Feature)

*This feature is planned but not yet implemented.*

In the future, you'll be able to specify different credentials per profile:

```json
{
  "profiles": {
    "work": {
      "jira_url": "https://work.atlassian.net",
      "credentials_env_prefix": "WORK_JIRA"
    },
    "personal": {
      "jira_url": "https://personal.atlassian.net",
      "credentials_env_prefix": "PERSONAL_JIRA"
    }
  }
}
```

## Verifying Environment Variables

Check if your environment variables are set correctly:

### Linux / macOS

```bash
echo $JIRA_API_TOKEN
echo $JIRA_EMAIL
```

### Windows PowerShell

```powershell
echo $env:JIRA_API_TOKEN
echo $env:JIRA_EMAIL
```

### Windows Command Prompt

```cmd
echo %JIRA_API_TOKEN%
echo %JIRA_EMAIL%
```

:::tip
If the commands return empty values, your environment variables aren't set.
:::

## Troubleshooting

### "Authentication failed" Error

**Cause:** Invalid credentials or missing environment variables.

**Solutions:**
1. Verify environment variables are set:
   ```bash
   echo $JIRA_EMAIL
   echo $JIRA_API_TOKEN
   ```

2. Check that `JIRA_EMAIL` matches your Jira account

3. Regenerate your API token if it's expired

4. Ensure you're running tedlt from a directory with a `.env` file (if using that approach)

### ".env file not loaded"

**Cause:** The `.env` file is not in the current directory or a parent directory.

**Solutions:**
1. Check your current directory:
   ```bash
   pwd  # Linux/macOS
   cd   # Windows
   ```

2. Verify `.env` exists:
   ```bash
   ls -la .env      # Linux/macOS
   dir .env         # Windows
   ```

3. Check file contents:
   ```bash
   cat .env         # Linux/macOS
   type .env        # Windows
   ```

### Environment Variables Not Updating

**Cause:** Variables are cached in your shell session.

**Solutions:**
1. Start a new terminal session

2. Reload your shell configuration:
   ```bash
   source ~/.bashrc  # or ~/.zshrc
   ```

3. Re-export the variables in the current session

## Security Best Practices

### Do's

✅ **Use `.env` files** for local development

✅ **Add `.env` to `.gitignore`** to prevent committing credentials

✅ **Rotate API tokens regularly** (every 6-12 months)

✅ **Use different tokens** for different machines/projects

✅ **Provide `.env.example`** files in shared repositories

### Don'ts

❌ **DON'T commit `.env` files** to version control

❌ **DON'T share API tokens** with others

❌ **DON'T put tokens in config files** (even temporarily)

❌ **DON'T hardcode credentials** in scripts

❌ **DON'T use the same token everywhere** (use different tokens per context)

## Example Setup

### For a Team Repository

1. Create `.env.example` with placeholders:
   ```bash
   # .env.example
   JIRA_API_TOKEN=your_api_token_here
   JIRA_EMAIL=your.email@example.com
   ```

2. Add `.env` to `.gitignore`:
   ```
   .env
   ```

3. Document in README:
   ```markdown
   ## Setup
   
   1. Copy `.env.example` to `.env`
   2. Fill in your Jira credentials
   3. Generate an API token at https://id.atlassian.com/manage-profile/security/api-tokens
   ```

4. Each developer creates their own `.env`:
   ```bash
   cp .env.example .env
   # Edit .env with your credentials
   ```

## Next Steps

- **[Configuration File](/tedlt/configuration/config-file/)** - Set up your config file
- **[Quick Start](/tedlt/getting-started/quick-start/)** - Complete setup guide
- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Start using tedlt