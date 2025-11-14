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

:::tip
The `fields` object is send as is to Jira API, this means you can copy POST bodies from Jira documentation or inspect existing tickets to directly build your profiles.
:::

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
        "assignee": { "id": "${team_lead}" },
        "components": [{ "id": "${frontend_component}" }]
      }
    }
  }
}
```

See [Property Templates](/tedlt/configuration/properties/) for more details.

## Next Steps

- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex profiles from simple ones
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in profiles
- **[Configuration Schema](/tedlt/reference/config-schema/)** - See all available options
