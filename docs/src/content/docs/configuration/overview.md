---
title: Configuration Overview
description: Understanding how tedlt configuration works
---

Configuration in tedlt is flexible and hierarchical, allowing you to set defaults globally while overriding them per-profile or per-command.

## Configuration Components

tedlt uses three main configuration components:

1. **Configuration File** (`~/tedlt.jsonc`) - Stores Jira instance details, defaults, and profile definitions
2. **Environment Variables** - Secures your API credentials
3. **CLI Arguments** - Provides command-specific overrides

## Configuration Priority

Settings are applied in this priority order (highest to lowest):

1. **CLI arguments** (`--jira-url`, `--project-key`)
2. **Profile settings** (from `--profile`)
3. **Default profile** (automatically applied)
4. **Top-level config file** settings
5. **Environment variables** (credentials only)

## Basic Structure

A minimal configuration file:

```json
{
  "jira_url": "https://yourcompany.atlassian.net",
  "project_key": "PROJ"
}
```

This is enough to start creating tickets.

## Complete Structure

A full configuration with all features:

```json
{
  "jira_url": "https://yourcompany.atlassian.net",
  "project_key": "MAIN",
  
  "properties": {
    "team_lead": "USER123",
    "default_priority": "3"
  },
  
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "${default_priority}" },
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

## Top-Level Fields

### `jira_url`

Your Jira instance URL. Can be overridden per-profile or with `--jira-url`.

```json
{
  "jira_url": "https://yourcompany.atlassian.net"
}
```

### `project_key`

The default project key for creating tickets. Can be overridden per-profile or with `--project-key`.

```json
{
  "project_key": "PROJ"
}
```

### `properties`

Reusable variables for use in profiles. Reference with `${property_name}` syntax.

```json
{
  "properties": {
    "team_lead": "USER123",
    "frontend_component": "10100"
  }
}
```

See [Property Templates](/tedlt/configuration/properties/) for usage details.

### `profiles`

Named configurations for different ticket types.

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" }
      }
    }
  }
}
```

See [Profiles](/tedlt/configuration/profiles/) for usage details.

## Configuration Workflow

### 1. Set Up Credentials

Create environment variables for authentication:

```bash
export JIRA_API_TOKEN="your_token_here"
export JIRA_EMAIL="your.email@example.com"
```

Or use a `.env` file. See [Environment Variables](/tedlt/configuration/environment/).

### 2. Initialize Configuration

Create the basic config file:

```bash
tedlt init --jira-url https://company.atlassian.net --project-key PROJ
```

### 3. Discover Field IDs

Find IDs for fields you want to configure:

```bash
# Project details (issue types, components, etc.)
tedlt info project PROJ

# Available fields for an issue type
tedlt info fields --project-key PROJ --issue-type 10001

# Inspect existing ticket
tedlt info ticket PROJ-123
```

### 4. Configure Profiles

Add profiles for different ticket types. See the [Configuration File](/tedlt/configuration/config-file/) guide for editing details.

### 5. Create Tickets

Use your configured profiles:

```bash
tedlt create "Fix bug" --profile bug
```

## How Configuration Merges

When using multiple configuration sources, tedlt merges them intelligently:

- **Scalars** (strings, numbers) - Later values replace earlier ones
- **Arrays** - Concatenated (parent + child elements)
- **Objects** - Deep-merged recursively

See [Profile Inheritance](/tedlt/configuration/inheritance/) for detailed merge behavior and examples.

## Multiple Jira Instances

Use profiles to work with different Jira instances:

```json
{
  "profiles": {
    "work": {
      "jira_url": "https://work.atlassian.net",
      "project_key": "WORK"
    },
    "personal": {
      "jira_url": "https://personal.atlassian.net",
      "project_key": "PERSONAL"
    }
  }
}
```

Then create tickets with:

```bash
tedlt create "Work task" --profile work
tedlt create "Personal task" --profile personal
```

## Next Steps

**Core Configuration:**
- [Environment Variables](/tedlt/configuration/environment/) - Set up API credentials
- [Configuration File](/tedlt/configuration/config-file/) - File location, format, and editing

**Advanced Features:**
- [Profiles](/tedlt/configuration/profiles/) - Create reusable ticket templates
- [Property Templates](/tedlt/configuration/properties/) - Use variables in configuration
- [Profile Inheritance](/tedlt/configuration/inheritance/) - Build complex configurations from simple ones

**Reference:**
- [Configuration Schema](/tedlt/reference/config-schema/) - Complete field reference and validation rules