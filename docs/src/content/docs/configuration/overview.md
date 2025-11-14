---
title: Configuration Overview
description: Understanding how tedlt configuration works
---

Configuration in tedlt is flexible and hierarchical, allowing you to set defaults globally while overriding them per-profile or per-command.

## Configuration Sources

tedlt reads configuration from multiple sources, applied in this priority order (highest to lowest):

1. **CLI arguments** (`--jira-url`, `--project-key`)
2. **Profile settings** (from `--profile`)
3. **Default profile** (automatically applied)
4. **Top-level config file** settings
5. **Environment variables** (credentials only)

## Configuration File Location

The configuration file is located at: `~/tedlt.jsonc`

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

Your Jira instance URL.

```json
{
  "jira_url": "https://yourcompany.atlassian.net"
}
```

This can be overridden per-profile or with `--jira-url`.

### `project_key`

The default project key for creating tickets.

```json
{
  "project_key": "PROJ"
}
```

This can be overridden per-profile or with `--project-key`.

### `properties`

Reusable variables for use in profiles.

```json
{
  "properties": {
    "team_lead": "USER123",
    "frontend_component": "10100"
  }
}
```

Reference with `${property_name}` syntax. See [Property Templates](/tedlt/configuration/properties/).

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

See [Profiles](/tedlt/configuration/profiles/) for details.

## Environment Variables

Credentials are read from environment variables:

- `JIRA_API_TOKEN`: Your Jira API token
- `JIRA_EMAIL`: Your Jira account email

### Using .env Files

Create a `.env` file in your project directory:

```bash
JIRA_API_TOKEN=your_token_here
JIRA_EMAIL=your.email@example.com
```

tedlt automatically loads this file when running commands from that directory.

See [Environment Variables](/tedlt/configuration/environment/) for details.

## Configuration Workflow

1. **Initialize**: Create the basic config file
   ```bash
   tedlt init --jira-url https://company.atlassian.net --project-key PROJ
   ```

2. **Discover**: Find IDs for fields you want to configure
   ```bash
   tedlt info project PROJ
   ```

3. **Configure**: Add profiles for different ticket types
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

4. **Use**: Create tickets with your profiles
   ```bash
   tedlt create "Fix bug" --profile bug
   ```

## How Merging Works

When you use multiple configuration sources, they are merged with these rules:

### Scalars (Strings, Numbers, Booleans)

Later values completely replace earlier ones:

```json
// Top-level config
{
  "project_key": "MAIN"
}

// Profile
{
  "profiles": {
    "other": {
      "project_key": "OTHER"  // Replaces "MAIN"
    }
  }
}
```

### Arrays

Arrays are concatenated:

```json
// Default profile
{
  "fields": {
    "labels": ["auto-created"]
  }
}

// Bug profile
{
  "fields": {
    "labels": ["bug"]
  }
}

// Result: ["auto-created", "bug"]
```

### Objects

Objects are deep-merged recursively:

```json
// Default profile
{
  "fields": {
    "customfield_1": {
      "property1": "default",
      "property2": "default"
    }
  }
}

// Bug profile
{
  "fields": {
    "customfield_1": {
      "property2": "bug",
      "property3": "bug"
    }
  }
}

// Result:
{
  "customfield_1": {
    "property1": "default",
    "property2": "bug",
    "property3": "bug"
  }
}
```

## Multiple Configuration Files

You can use different configuration files for different contexts:

### Per-Project Configuration

Create a `tedlt.jsonc` in your project directory (feature not yet implemented, planned for future versions).

Currently, use profiles to separate contexts:

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

## Next Steps

- **[Environment Variables](/tedlt/configuration/environment/)** - Set up credentials
- **[Configuration File](/tedlt/configuration/config-file/)** - Detailed file structure
- **[Profiles](/tedlt/configuration/profiles/)** - Create reusable templates
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in configuration
