---
title: Configuration File
description: Detailed guide to the tedlt configuration file structure
---

The configuration file stores your Jira instance details, default settings, and profile definitions. It's a JSON file located in your home directory.

## File Location

The configuration file is created by `tedlt init` at: `~/tedlt.jsonc`.

## File Format

The configuration file is JSONC (JSON with comments).

## Configuration Schema

### Root Object

```json
{
  "jira_url": "string",
  "project_key": "string",
  "properties": {},
  "profiles": {}
}
```

All fields are optional, but `jira_url` and `project_key` are typically required for most operations.

### Top-Level Fields

#### `jira_url`

**Type:** String  
**Required:** No (but recommended)  
**Description:** Your Jira instance URL.

```json
{
  "jira_url": "https://yourcompany.atlassian.net"
}
```

**Valid formats:**

- `https://yourcompany.atlassian.net`
- `https://jira.yourcompany.com`
- Any valid Jira base URL

#### `project_key`

**Type:** String  
**Required:** No (but recommended)  
**Description:** Default project key for creating tickets.

```json
{
  "project_key": "PROJ"
}
```

**Format:**

- Usually 2-10 uppercase letters
- Examples: `PROJ`, `KAN`, `DEV`, `MYTEAM`

#### `properties`

**Type:** Object  
**Required:** No  
**Description:** Reusable variables for use in profiles.

```json
{
  "properties": {
    "team_lead": "USER123",
    "default_priority": "3",
    "frontend_component": "10100",
    "issueTypes": {
      "story": 10001
    }
  }
}
```

Properties are referenced in profiles using `${property_name}` syntax. Nested properties are supported (e.g., `${issueTypes.story}`).

See [Property Templates](/tedlt/configuration/properties/) for details.

#### `profiles`

**Type:** Object  
**Required:** No  
**Description:** Named configurations for different ticket types.

```json
{
  "profiles": {
    "default": {},
    "bug": {},
    "feature": {}
  }
}
```

See [Profiles](/tedlt/configuration/profiles/) for details.

## Profile Structure

Each profile can contain:

### Profile Fields

```json
{
  "profiles": {
    "bug": {
      "jira_url": "string",
      "project_key": "string",
      "inherits": ["profile1", "profile2"],
      "fields": {}
    }
  }
}
```

#### `jira_url` (in profile)

Override the Jira URL for this profile.

```json
{
  "profiles": {
    "personal": {
      "jira_url": "https://personal.atlassian.net",
      "project_key": "PERSONAL"
    }
  }
}
```

#### `project_key` (in profile)

Override the project key for this profile.

```json
{
  "profiles": {
    "backend": {
      "project_key": "BACKEND"
    }
  }
}
```

#### `inherits`

**Type:** Array of strings  
**Description:** List of profiles to inherit from.

```json
{
  "profiles": {
    "bug-critical": {
      "inherits": ["bug", "urgent"]
    }
  }
}
```

Profiles are merged left-to-right. See [Profile Inheritance](/tedlt/configuration/inheritance/).

**Restrictions:**

- The `default` profile cannot have an `inherits` field
- No circular dependencies allowed

#### `fields`

**Type:** Object  
**Description:** Jira field values to include when creating tickets.

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"],
        "components": [{ "id": "10100" }],
        "customfield_10050": "value"
      }
    }
  }
}
```

## Field Types

Different Jira fields require different value formats.

### Simple Text Fields

```json
{
  "fields": {
    "summary": "Ticket title",
    "description": "Detailed description",
    "customfield_10060": "Custom text value"
  }
}
```

Note: `summary` is set via the CLI title argument, not the config.

### ID-Based Fields

Most structured fields use an object with an `id` property:

```json
{
  "fields": {
    "issuetype": { "id": "10004" },
    "priority": { "id": "2" },
    "assignee": { "id": "USER123" }
  }
}
```

### Array Fields

```json
{
  "fields": {
    "labels": ["bug", "critical"],
    "components": [{ "id": "10100" }, { "id": "10101" }],
    "fixVersions": [{ "id": "10200" }]
  }
}
```

### Nested Object Fields

```json
{
  "fields": {
    "customfield_10070": {
      "value": "Option 1"
    },
    "customfield_10071": {
      "id": "10300",
      "value": "Nested value"
    }
  }
}
```

### Epic Link

Epic link field varies by Jira instance. Find the field name first:

```bash
tedlt info fields --project-key PROJ --issue-type 10001
```

Then use it:

```json
{
  "fields": {
    "customfield_10050": "PROJ-100"
  }
}
```

## Common Field Examples

### Complete Bug Profile

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug", "needs-triage"],
        "components": [{ "id": "10100" }],
        "assignee": { "id": "USER123" }
      }
    }
  }
}
```

### Story with Epic Link

```json
{
  "profiles": {
    "story": {
      "fields": {
        "issuetype": { "id": "10002" },
        "priority": { "id": "3" },
        "labels": ["story"],
        "customfield_10050": "PROJ-100"
      }
    }
  }
}
```

### Task with Custom Fields

```json
{
  "profiles": {
    "task": {
      "fields": {
        "issuetype": { "id": "10001" },
        "priority": { "id": "3" },
        "labels": ["task"],
        "customfield_10060": "Sprint 1",
        "customfield_10061": { "value": "Team A" }
      }
    }
  }
}
```

## Complete Configuration Examples

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
    "default": {
      "fields": {
        "priority": { "id": "3" }
      }
    },
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
  "jira_url": "https://work.atlassian.net",
  "project_key": "WORK",
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" }
      }
    },
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
        "priority": { "id": "3" },
        "labels": ["auto-created"]
      }
    },
    "bug-base": {
      "fields": {
        "issuetype": { "id": "10004" },
        "labels": ["bug"]
      }
    },
    "bug": {
      "inherits": ["bug-base"],
      "fields": {
        "priority": { "id": "2" }
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

## Editing the Configuration File

### Manual Editing

Open the file in your favorite text editor:

```bash
# Linux/macOS
nano ~/tedlt.jsonc
vim ~/tedlt.jsonc
code ~/tedlt.jsonc

# Windows
notepad %USERPROFILE%\tedlt.jsonc
code %USERPROFILE%\tedlt.jsonc
```

### Using jq (Linux/macOS)

Add a new profile:

```bash
cat ~/tedlt.jsonc | jq '.profiles.new_profile = {"fields": {"priority": {"id": "2"}}}' > ~/tedlt.jsonc
```

### Backup Before Editing

Always backup before making major changes:

```bash
# Linux/macOS
cp ~/tedlt.jsonc ~/tedlt.jsonc.backup

# Windows
copy %USERPROFILE%\tedlt.jsonc %USERPROFILE%\tedlt.jsonc.backup
```

## Validation

tedlt validates your configuration when loading it.

### Common Validation Errors

**Invalid JSON syntax:**

```
Error: Failed to parse config file: expected `,` at line 5
```

Fix: Check for missing commas, quotes, or brackets.

**Profile not found:**

```
Error: Profile 'nonexistent' referenced in inherits but not defined
```

Fix: Ensure all profiles in `inherits` arrays exist.

**Circular dependency:**

```
Error: Circular dependency detected: a -> b -> a
```

Fix: Remove circular inheritance chains.

**Default profile with inherits:**

```
Error: Default profile cannot have 'inherits' field
```

Fix: Remove the `inherits` field from the `default` profile.

### Testing Your Configuration

Test with verbose mode to see how configuration is resolved:

```bash
tedlt create "Test" --profile bug --verbose
```

This shows:

- Which profiles are loaded
- How they're merged
- Final configuration values
- API request payload

## Next Steps

- **[Profiles](/tedlt/configuration/profiles/)** - Learn about profile configuration
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables
- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex configs
- **[Configuration Schema](/tedlt/reference/config-schema/)** - Complete reference
