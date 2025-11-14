---
title: Configuration File
description: Managing your tedlt configuration file
---

The configuration file stores your Jira instance details, default settings, and profile definitions.

## File Location

The configuration file is located at: `~/tedlt.jsonc`

## File Format

The configuration file uses **JSONC** (JSON with Comments), which allows you to add comments for documentation:

```jsonc
{
  // Your Jira instance
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ",
  
  // Reusable variables
  "properties": {
    "team_lead": "USER123"  // Updated quarterly
  },
  
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" }
      }
    }
  }
}
```

## Creating the Configuration File

The easiest way to create a configuration file:

```bash
tedlt init --jira-url https://company.atlassian.net --project-key PROJ
```

This creates a basic configuration file with your Jira URL and project key.

## Basic Structure

A minimal configuration:

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ"
}
```

A complete configuration with all features:

```json
{
  "jira_url": "https://company.atlassian.net",
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

See the [Configuration Schema](/tedlt/reference/config-schema/) for complete field definitions.

## Configuration Sections

### Top-Level Fields

- **`jira_url`** - Your Jira instance URL
- **`project_key`** - Default project for creating tickets
- **`properties`** - Reusable variables (see [Property Templates](/tedlt/configuration/properties/))
- **`profiles`** - Named ticket templates (see [Profiles](/tedlt/configuration/profiles/))

### Profile Fields

Each profile can contain:

- **`jira_url`** - Override Jira URL for this profile
- **`project_key`** - Override project key for this profile
- **`inherits`** - List of profiles to inherit from (see [Inheritance](/tedlt/configuration/inheritance/))
- **`fields`** - Jira field values (issue type, priority, labels, etc.)

See the [Configuration Schema](/tedlt/reference/config-schema/) for complete field reference and formats.

## Finding Field Values

Before configuring profiles, you need to find the correct field IDs.

### Project Information

```bash
tedlt info project PROJ
```

This shows:
- Issue type IDs and names
- Component IDs and names
- Version IDs and names
- Project metadata

### Available Fields

```bash
tedlt info fields --project-key PROJ --issue-type 10001
```

This shows all available fields and their types for a specific issue type.

### Existing Tickets

```bash
tedlt info ticket PROJ-123
```

Copy field values from a well-configured ticket into your profile.

## Common Configuration Examples

### Simple Setup

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "MAIN",
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" },
        "labels": ["auto-created"]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "labels": ["bug"]
      }
    }
  }
}
```

### Multi-Team Setup

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "SHARED",
  "properties": {
    "frontend_component": "10100",
    "backend_component": "10101"
  },
  "profiles": {
    "frontend": {
      "fields": {
        "components": [{ "id": "${frontend_component}" }],
        "labels": ["frontend"]
      }
    },
    "backend": {
      "fields": {
        "components": [{ "id": "${backend_component}" }],
        "labels": ["backend"]
      }
    }
  }
}
```

### Multi-Instance Setup

```json
{
  "profiles": {
    "work": {
      "jira_url": "https://work.atlassian.net",
      "project_key": "WORK",
      "fields": {
        "labels": ["work"]
      }
    },
    "personal": {
      "jira_url": "https://personal.atlassian.net",
      "project_key": "PERSONAL",
      "fields": {
        "labels": ["personal"]
      }
    }
  }
}
```

### With Inheritance

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ",
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" }
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "labels": ["bug"]
      }
    },
    "bug-critical": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "1" },
        "labels": ["critical", "urgent"]
      }
    }
  }
}
```

## Next Steps

**Usage Guides:**
- [Profiles](/tedlt/configuration/profiles/) - Learn about profile configuration
- [Property Templates](/tedlt/configuration/properties/) - Use variables in profiles
- [Profile Inheritance](/tedlt/configuration/inheritance/) - Build complex configurations

**Reference:**
- [Configuration Schema](/tedlt/reference/config-schema/) - Complete field reference and validation rules
