---
title: Environment Variables
description: Configure Jira credentials with environment variables
---

Environment variables are used to securely store your Jira credentials.

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

### Option 1: Shell Environment Variables

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

### Option 2: `.env` File

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

## Next Steps

- **[Configuration File](/tedlt/configuration/config-file/)** - Set up your config file
- **[Quick Start](/tedlt/getting-started/quick-start/)** - Complete setup guide
- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Start using tedlt
