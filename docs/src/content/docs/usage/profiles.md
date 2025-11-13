---
title: Using Profiles
description: Learn how to configure and use profiles for different ticket types
---

Profiles are pre-configured templates for creating different types of tickets. They let you define common field values once and reuse them across many tickets.

## What Are Profiles?

A profile is a named configuration that specifies:
- Which Jira instance to use
- Which project to create tickets in
- Default field values (issue type, priority, labels, etc.)

Instead of specifying these values every time you create a ticket, you reference a profile by name.

## Basic Profile Configuration

Profiles are defined in your configuration file under the `profiles` key:

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "MAIN",
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug", "needs-triage"]
      }
    }
  }
}
```

Use this profile:

```bash
tedlt create "Login error" --profile bug
```

## The Default Profile

The `default` profile is special—it's automatically applied to **every** ticket, even if you don't specify `--profile default`.

```json
{
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" },
        "labels": ["auto-created"],
        "components": [{ "id": "10000" }]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" }
      }
    }
  }
}
```

When you run:
```bash
tedlt create "Fix bug" --profile bug
```

The ticket gets fields from both `default` and `bug` profiles merged together.

:::tip
Use the `default` profile for settings that apply to **all** tickets in your project, like default priority, common labels, or team component IDs.
:::

## Profile Fields

### Top-Level Fields

Profiles can override top-level configuration:

```json
{
  "profiles": {
    "other-instance": {
      "jira_url": "https://other-company.atlassian.net",
      "project_key": "OTHER"
    }
  }
}
```

### Jira Fields

The `fields` object contains Jira field values that will be sent in the API request:

```json
{
  "profiles": {
    "feature": {
      "fields": {
        "issuetype": { "id": "10001" },
        "priority": { "id": "3" },
        "labels": ["feature", "enhancement"],
        "components": [{ "id": "10100" }],
        "customfield_10050": "custom-value"
      }
    }
  }
}
```

## Common Field Examples

### Issue Type

```json
{
  "fields": {
    "issuetype": { "id": "10004" }
  }
}
```

Use `tedlt info project PROJ` to find issue type IDs.

### Priority

```json
{
  "fields": {
    "priority": { "id": "2" }
  }
}
```

Common priority IDs:
- `1`: Highest
- `2`: High
- `3`: Medium
- `4`: Low
- `5`: Lowest

### Labels

```json
{
  "fields": {
    "labels": ["bug", "critical", "security"]
  }
}
```

Labels are combined when profiles are merged.

### Components

```json
{
  "fields": {
    "components": [
      { "id": "10100" },
      { "id": "10101" }
    ]
  }
}
```

Use `tedlt info project PROJ` to find component IDs.

### Assignee

```json
{
  "fields": {
    "assignee": { "id": "USER123" }
  }
}
```

### Custom Fields

```json
{
  "fields": {
    "customfield_10050": "text-value",
    "customfield_10051": { "id": "10200" },
    "customfield_10052": ["option1", "option2"]
  }
}
```

Use `tedlt info fields --project-key PROJ --issue-type 10001` to discover custom field names and types.

## Using Multiple Profiles

Combine profiles by specifying `--profile` multiple times:

```bash
tedlt create "Urgent bug" --profile bug --profile urgent
```

Profiles are merged **left-to-right**, meaning later profiles override earlier ones:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "priority": { "id": "3" },
        "labels": ["bug"]
      }
    },
    "urgent": {
      "fields": {
        "priority": { "id": "1" },
        "labels": ["urgent"]
      }
    }
  }
}
```

Result when using both profiles:
- **Priority**: `1` (from `urgent`, overrides `bug`)
- **Labels**: `["bug", "urgent"]` (arrays are concatenated)

## Profile Inheritance

Profiles can inherit from other profiles using the `inherits` field:

```json
{
  "profiles": {
    "bug-base": {
      "fields": {
        "issuetype": { "id": "10004" },
        "labels": ["bug"]
      }
    },
    "bug-critical": {
      "inherits": ["bug-base"],
      "fields": {
        "priority": { "id": "1" },
        "labels": ["critical"]
      }
    }
  }
}
```

Using `--profile bug-critical` gives you:
- Issue type from `bug-base`
- Labels from both: `["bug", "critical"]`
- Priority from `bug-critical`

See [Profile Inheritance](/tedlt/configuration/inheritance/) for detailed information.

## Property Templates

Use property templates to avoid repeating values:

```json
{
  "properties": {
    "team_lead": "USER123",
    "frontend_component": "10100"
  },
  "profiles": {
    "frontend": {
      "fields": {
        "assignee": { "id": "{{team_lead}}" },
        "components": [{ "id": "{{frontend_component}}" }]
      }
    }
  }
}
```

See [Property Templates](/tedlt/configuration/properties/) for more details.

## Real-World Profile Examples

### Bug Tracking Workflow

```json
{
  "profiles": {
    "default": {
      "fields": {
        "components": [{ "id": "10000" }],
        "labels": ["auto-created"]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "3" },
        "labels": ["bug"]
      }
    },
    "bug-high": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "2" }
      }
    },
    "bug-critical": {
      "inherits": ["bug-high"],
      "fields": {
        "priority": { "id": "1" },
        "labels": ["critical", "hotfix"]
      }
    }
  }
}
```

Usage:
```bash
tedlt create "Minor UI glitch" --profile bug
tedlt create "Payment processing error" --profile bug-high
tedlt create "Data loss bug" --profile bug-critical
```

### Multi-Team Setup

```json
{
  "properties": {
    "frontend_team": "10100",
    "backend_team": "10101",
    "devops_team": "10102"
  },
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" }
      }
    },
    "frontend": {
      "fields": {
        "components": [{ "id": "{{frontend_team}}" }],
        "labels": ["frontend"]
      }
    },
    "backend": {
      "fields": {
        "components": [{ "id": "{{backend_team}}" }],
        "labels": ["backend"]
      }
    },
    "devops": {
      "fields": {
        "components": [{ "id": "{{devops_team}}" }],
        "labels": ["devops", "infrastructure"]
      }
    }
  }
}
```

Usage:
```bash
tedlt create "Update button styles" --profile frontend
tedlt create "Optimize database query" --profile backend
tedlt create "Deploy new service" --profile devops
```

### Multi-Instance Support

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

Usage:
```bash
tedlt create "Work task" --profile work
tedlt create "Side project feature" --profile personal
```

## Best Practices

### Keep Profiles Focused

Each profile should represent one concept or dimension:

```json
// ✅ Good: Focused profiles
{
  "profiles": {
    "bug": { "fields": { "issuetype": { "id": "10004" } } },
    "high-priority": { "fields": { "priority": { "id": "2" } } },
    "frontend": { "fields": { "components": [{ "id": "10100" }] } }
  }
}

// Then combine as needed:
// tedlt create "Issue" --profile bug --profile high-priority --profile frontend
```

### Use the Default Profile

Put common settings in the `default` profile to avoid repetition:

```json
{
  "profiles": {
    "default": {
      "fields": {
        "components": [{ "id": "10000" }],
        "labels": ["auto-created"]
      }
    }
  }
}
```

### Name Profiles Clearly

Use descriptive names that indicate what the profile does:

```json
{
  "profiles": {
    "bug-critical": {},      // ✅ Clear
    "story-frontend": {},    // ✅ Clear
    "urgent-security": {},   // ✅ Clear
    "profile1": {},          // ❌ Unclear
    "temp": {}               // ❌ Unclear
  }
}
```

### Document Your Profiles

Add comments to explain profile purposes (if using JSONC format):

```jsonc
{
  "profiles": {
    // Used for all production bugs requiring immediate attention
    "bug-critical": {
      "fields": {
        "priority": { "id": "1" },
        "labels": ["critical", "production"]
      }
    }
  }
}
```

## Discovering Field IDs

Before configuring profiles, you need to find the correct field IDs:

### Find Issue Type IDs
```bash
tedlt info project PROJ
```

### Find Custom Field Names
```bash
tedlt info fields --project-key PROJ --issue-type 10001
```

### Inspect Existing Tickets
```bash
tedlt info ticket PROJ-123
```

This shows all fields and their values, which you can copy into your profiles.

## Next Steps

- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex profiles from simple ones
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in profiles
- **[Configuration Schema](/tedlt/reference/config-schema/)** - See all available options