---
title: Profiles
description: Create reusable templates for different ticket types
---

Profiles are the core of tedlt's configuration system. They allow you to define reusable templates for different types of tickets, so you don't have to specify the same field values every time.

## What Are Profiles?

A profile is a named set of configuration values that defines:

- Which Jira instance to use
- Which project to create tickets in
- Field values (issue type, priority, labels, etc.)

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
    },
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

## Using Profiles

Use a profile with the `--profile` flag:

```bash
tedlt create "Login error" --profile bug
tedlt create "Add user profile page" --profile story
```

## Profile Structure

Each profile can contain:

### `fields`

Jira field values to include when creating tickets:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"],
        "components": [{ "id": "10100" }],
        "assignee": { "id": "USER123" }
      }
    }
  }
}
```

See the [Configuration Schema](/tedlt/reference/config-schema/) for all available field types and formats.

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

**Use case:** Working with multiple Jira instances.

### `project_key`

Override the project key for this profile:

```json
{
  "profiles": {
    "backend": {
      "project_key": "BACKEND"
    },
    "frontend": {
      "project_key": "FRONTEND"
    }
  }
}
```

**Use case:** Creating tickets in different projects.

### `inherits`

Inherit settings from other profiles:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "3" }
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

See [Profile Inheritance](/tedlt/configuration/inheritance/) for details.

## The Default Profile

The `default` profile is special—it's **automatically applied to all tickets**, even when you don't specify it.

When you use `--profile bug`, you get fields from both `default` and `bug` merged together.

:::tip
Use the `default` profile for settings that should apply to all tickets, like default priority or common labels.
:::

:::caution
The `default` profile cannot have an `inherits` field.
:::

## Multiple Profiles

Combine multiple profiles by specifying `--profile` multiple times:

```bash
tedlt create "Urgent bug" --profile bug --profile urgent
```

Profiles are merged left-to-right, with later profiles overriding earlier ones.

See [Profile Inheritance](/tedlt/configuration/inheritance/) for detailed merge behavior.

## Discovering Field Values

Before configuring profiles, you need to find the correct field IDs.

See the [Configuration Schema](/tedlt/reference/config-schema/) for complete field reference.

## Next Steps

**Advanced Features:**

- [Profile Inheritance](/tedlt/configuration/inheritance/) - Build complex profiles from simple ones
- [Property Templates](/tedlt/configuration/properties/) - Use variables in profiles

**Reference:**

- [Configuration Schema](/tedlt/reference/config-schema/) - Complete field reference and formats

**Usage:**

- [Creating Tickets](/tedlt/usage/creating-tickets/) - Use your configured profiles
