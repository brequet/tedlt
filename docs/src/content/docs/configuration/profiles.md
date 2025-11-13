---
title: Profiles
description: Configure profiles for different ticket types
---

Profiles are the core of tedlt's configuration system. They allow you to define reusable templates for different types of tickets, so you don't have to specify the same field values every time.

## What Are Profiles?

A profile is a named set of configuration values that defines:
- Which Jira instance to use
- Which project to create tickets in
- Field values (issue type, priority, labels, etc.)

## Why Use Profiles?

Without profiles, you'd need to specify every field value for each ticket:

```bash
# Tedious and error-prone
tedlt create "Fix bug" --issuetype 10004 --priority 2 --label bug --component 10100
```

With profiles, you define these values once and reuse them:

```bash
# Simple and consistent
tedlt create "Fix bug" --profile bug
```

## Defining Profiles

Profiles are defined in the `profiles` section of your configuration file:

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ",
  "profiles": {
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

## Using Profiles

Use a profile with the `--profile` flag:

```bash
tedlt create "Login error" --profile bug
```

## Profile Structure

A profile can contain the following fields:

### `jira_url`

Override the Jira instance URL for this profile:

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

### `project_key`

Override the project key for this profile:

```json
{
  "profiles": {
    "backend": {
      "project_key": "BACKEND"
    }
  }
}
```

### `inherits`

Inherit settings from other profiles:

```json
{
  "profiles": {
    "bug-critical": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "1" }
      }
    }
  }
}
```

See [Profile Inheritance](/tedlt/configuration/inheritance/) for details.

### `fields`

The `fields` object contains Jira field values:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"],
        "components": [{ "id": "10100" }]
      }
    }
  }
}
```

## The Default Profile

The `default` profile is automatically applied to all tickets:

```json
{
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" },
        "labels": ["auto-created"]
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

When you use `--profile bug`, you get fields from both `default` and `bug` merged together.

:::tip
Use the `default` profile for settings that should apply to all tickets, like default priority or common labels.
:::

:::caution
The `default` profile cannot have an `inherits` field.
:::

## Common Profile Examples

### Bug Profile

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"],
        "components": [{ "id": "10100" }]
      }
    }
  }
}
```

Usage:
```bash
tedlt create "Login fails after password reset" --profile bug
```

### Story Profile

```json
{
  "profiles": {
    "story": {
      "fields": {
        "issuetype": { "id": "10002" },
        "priority": { "id": "3" },
        "labels": ["story"]
      }
    }
  }
}
```

Usage:
```bash
tedlt create "Add user profile page" --profile story
```

### Task Profile

```json
{
  "profiles": {
    "task": {
      "fields": {
        "issuetype": { "id": "10001" },
        "priority": { "id": "3" },
        "labels": ["task"]
      }
    }
  }
}
```

Usage:
```bash
tedlt create "Update documentation" --profile task
```

### Critical Bug Profile

```json
{
  "profiles": {
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

Usage:
```bash
tedlt create "Production database down" --profile bug-critical
```

## Multiple Profiles

Combine multiple profiles by specifying `--profile` multiple times:

```bash
tedlt create "Urgent bug" --profile bug --profile urgent
```

Profiles are merged left-to-right, with later profiles overriding earlier ones.

Example configuration:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
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

Using `--profile bug --profile urgent` gives you:
- Issue type: `10004` (from bug)
- Priority: `1` (from urgent, overrides bug)
- Labels: `["bug", "urgent"]` (arrays are concatenated)

## Discovering Field Values

Before configuring profiles, you need to find the correct field IDs.

### Find Issue Type IDs

```bash
tedlt info project PROJ
```

Look for the `issueTypes` array:
```json
{
  "issueTypes": [
    { "id": "10001", "name": "Task" },
    { "id": "10004", "name": "Bug" },
    { "id": "10002", "name": "Story" }
  ]
}
```

### Find Component IDs

```bash
tedlt info project PROJ
```

Look for the `components` array:
```json
{
  "components": [
    { "id": "10100", "name": "Frontend" },
    { "id": "10101", "name": "Backend" }
  ]
}
```

### Find Custom Field Names

```bash
tedlt info fields --project-key PROJ --issue-type 10001
```

This shows all available fields and their types.

### Inspect Existing Tickets

```bash
tedlt info ticket PROJ-123
```

Copy field values from a well-configured ticket into your profile.

## Field Types Reference

### Issue Type

```json
{
  "fields": {
    "issuetype": { "id": "10004" }
  }
}
```

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
    "labels": ["bug", "critical", "frontend"]
  }
}
```

Labels are arrays and are concatenated when profiles are merged.

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

### Assignee

```json
{
  "fields": {
    "assignee": { "id": "USER123" }
  }
}
```

Find user IDs by inspecting existing tickets.

### Epic Link

The epic link field name varies by Jira instance. Find it using:

```bash
tedlt info fields --project-key PROJ --issue-type 10002
```

Then use it:

```json
{
  "fields": {
    "customfield_10050": "PROJ-100"
  }
}
```

### Custom Fields

```json
{
  "fields": {
    "customfield_10060": "text value",
    "customfield_10061": { "id": "10200" },
    "customfield_10062": { "value": "Option 1" }
  }
}
```

Use `tedlt info fields` to discover custom field names and value formats.

## Best Practices

### Use Descriptive Names

Choose clear, descriptive profile names:

✅ Good:
- `bug-critical`
- `frontend-story`
- `devops-task`

❌ Bad:
- `profile1`
- `temp`
- `test`

### Keep Profiles Focused

Each profile should represent one concept:

✅ Good:
```json
{
  "profiles": {
    "bug": { },
    "critical": { },
    "frontend": { }
  }
}
```

Then combine: `--profile bug --profile critical --profile frontend`

❌ Bad:
```json
{
  "profiles": {
    "bug-critical-frontend": { }
  }
}
```

### Use the Default Profile

Put common settings in the `default` profile:

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

### Use Inheritance

Build on existing profiles instead of duplicating:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "3" }
      }
    },
    "bug-high": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "2" }
      }
    }
  }
}
```

### Use Property Templates

Avoid repeating values:

```json
{
  "properties": {
    "team_lead": "USER123",
    "frontend_component": "10100"
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "{{team_lead}}" },
        "components": [{ "id": "{{frontend_component}}" }]
      }
    }
  }
}
```

### Test with Verbose Mode

Verify your profile configuration:

```bash
tedlt create "Test ticket" --profile bug --verbose
```

This shows how profiles are merged and what values are sent to Jira.

## Complete Example

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
        "priority": { "id": "{{default_priority}}" },
        "labels": ["auto-created"]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"]
      }
    },
    "bug-critical": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "1" },
        "assignee": { "id": "{{team_lead}}" },
        "labels": ["critical", "urgent"]
      }
    },
    "story": {
      "fields": {
        "issuetype": { "id": "10002" },
        "labels": ["story"]
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

## Next Steps

- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex profiles from simple ones
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in profiles
- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Use your configured profiles