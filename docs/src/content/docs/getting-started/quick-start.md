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

### Option A: Using a `.env` file (Recommended)

Create a `.env` file in the directory where you'll run tedlt:

```bash
# .env
JIRA_API_TOKEN=your_api_token_here
JIRA_EMAIL=your.email@example.com
```

tedlt will automatically load this file when you run commands from that directory.

### Option B: Export environment variables

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

:::tip
The `.env` file approach is recommended because:
- Credentials are project-specific
- No need to set variables in every shell session
- Easy to share setup with team members (just don't commit the actual `.env` file!)
:::

## Step 3: Initialize Configuration

Create a configuration file with your Jira instance details:

```bash
tedlt init
```

This creates a configuration file at:
- **Windows**: `C:\Users\YourName\.tedlt\config.json`
- **macOS/Linux**: `~/.tedlt/config.json`

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

## Step 4: Discover Your Project

Before creating tickets, explore your Jira project to understand its structure:

```bash
tedlt info project PROJ
```

This shows:
- Available issue types (Task, Bug, Story, etc.) and their IDs
- Components and their IDs
- Versions and their IDs
- Other project metadata

Example output:
```json
{
  "key": "PROJ",
  "name": "My Project",
  "issueTypes": [
    { "id": "10001", "name": "Task" },
    { "id": "10004", "name": "Bug" },
    { "id": "10002", "name": "Story" }
  ],
  "components": [
    { "id": "10100", "name": "Frontend" },
    { "id": "10101", "name": "Backend" }
  ]
}
```

:::tip
Save these IDs—you'll need them when configuring profiles!
:::

## Step 5: Create Your First Ticket

Create a simple ticket with just a title:

```bash
tedlt create "Fix login page error"
```

If everything is configured correctly, tedlt will:
1. Connect to your Jira instance
2. Create a ticket in your default project
3. Display the ticket key (e.g., `PROJ-123`)

Success! You've created your first ticket.

## Step 6: Configure Profiles (Optional)

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

## Common Commands

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

## Troubleshooting

### "Authentication failed"
- Verify your API token is correct
- Check that `JIRA_EMAIL` matches your Jira account email
- Ensure environment variables are loaded (try running from the directory with your `.env` file)

### "Project not found"
- Verify the project key in your config file
- Check that you have access to the project in Jira

### "Field required" errors
- Use `tedlt info fields` to discover which fields are required
- Add required fields to your profile configuration

### "Command not found"
- Ensure tedlt is in your PATH
- Try running with the full path to the binary

## Next Steps

Now that you can create tickets, explore more features:

- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Learn all the ways to create tickets
- **[Using Profiles](/tedlt/usage/profiles/)** - Master profile configuration
- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex profiles from simple ones
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in your configuration