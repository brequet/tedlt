---
title: Quick Start
description: Get up and running with tedlt in minutes
---

This guide will walk you through setting up tedlt and creating your first Jira ticket.

## Prerequisites

- tedlt installed on your system ([Installation Guide](/tedlt/getting-started/installation/))
- A Jira account with API access
- Your Jira instance URL (e.g., `https://yourcompany.atlassian.net`)

## Step 1: Generate a Jira API Token

tedlt uses Jira's REST API, which requires an API token for authentication.

1. Go to [https://id.atlassian.com/manage-profile/security/api-tokens](https://id.atlassian.com/manage-profile/security/api-tokens)
2. Click **Create API token**
3. Give it a label (e.g., "tedlt CLI")
4. Click **Create**
5. Copy the token (you won't be able to see it again)

## Step 2: Set Up Environment Variables

tedlt reads your Jira credentials from environment variables. You have two options:

### Option A: Export environment variables

Set environment variables in your shell:

```bash
# Linux/macOS
export JIRA_API_TOKEN="your_api_token_here"
export JIRA_EMAIL="your.email@example.com"

# Windows (PowerShell)
$env:JIRA_API_TOKEN="your_api_token_here"
$env:JIRA_EMAIL="your.email@example.com"

# Windows (Command Prompt)
set JIRA_API_TOKEN=your_api_token_here
set JIRA_EMAIL=your.email@example.com
```

### Option B: Using a `.env` file (Recommended if using multiple Jira instances)

Create a `.env` file in the directory where you'll run tedlt:

```bash
# .env
JIRA_API_TOKEN=your_api_token_here
JIRA_EMAIL=your.email@example.com
```

tedlt will automatically load this file when you run commands from that directory.

:::tip
The `.env` file approach let the user set up multiple workspaces with different configuration, allowing
for project-specific Jira credentials configuration.
:::

## Step 3: Initialize Configuration

Create a configuration file with your Jira instance details:

```bash
tedlt init
```

This creates a configuration file at: `~/.tedlt.jsonc`

You'll be prompted to enter:
- **Jira URL**: Your Jira instance URL (e.g., `https://yourcompany.atlassian.net`)
- **Project Key**: Your default project key (e.g., `PROJ`)

Alternatively, provide these values directly:

```bash
tedlt init --jira-url https://yourcompany.atlassian.net --project-key PROJ
```

:::note
If a config file already exists, use `--force` to overwrite it:
```bash
tedlt init --force
```
:::

## Step 4: Create Your First Ticket

Create a simple ticket with just a title:

```bash
tedlt create "Fix login page error"
```

Success! You've created your first ticket.

## Step 5: Go further and configure Profiles

Profiles let you create different types of tickets with pre-configured fields. Edit your config file:

```json
{
  "jira_url": "https://yourcompany.atlassian.net",
  "project_key": "PROJ",
  "profiles": {
    "default": {
      "fields": {
        "issuetype": { "id": "10001" },
        "priority": { "id": "3" },
        "labels": ["auto-created"]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"]
      }
    }
  }
}
```

Now create a bug ticket:

```bash
tedlt create "Null pointer exception in user service" --profile bug
```

:::tip
To get your board configurable fields and values, use the `tedlt info` commands.
:::

## Common Commands Examples

Here are the most common commands you'll use:

```bash
# Create a ticket with default profile
tedlt create "Update documentation"

# Create a ticket with a specific profile
tedlt create "Fix memory leak" --profile bug

# Create a ticket with multiple profiles (merged left-to-right)
tedlt create "Critical bug" --profile bug --profile urgent

# View project information
tedlt info project PROJ

# View ticket details
tedlt info ticket PROJ-123

# List all boards
tedlt info boards

# Find epics in a project
tedlt info epics --project-key PROJ

# Discover available fields for an issue type
tedlt info fields --project-key PROJ --issue-type 10001
```

## Next Steps

Now that you can create tickets, explore more features:

- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Learn all the ways to create tickets
- **[Using Profiles](/tedlt/usage/profiles/)** - Master profile configuration
- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex profiles from simple ones
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in your configuration
