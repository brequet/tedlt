---
title: Creating Tickets
description: Learn how to create Jira tickets with tedlt
---

Creating tickets with tedlt is simple and flexible. At its core, you just need a title, but you can customize ticket creation using profiles and CLI overrides.

## Basic Usage

The simplest way to create a ticket:

```bash
tedlt create "Fix login page error"
```

This creates a ticket in your default project with the title "Fix login page error".

## Using Profiles

Profiles allow you to pre-configure fields for different types of tickets. Specify a profile with the `--profile` flag:

```bash
tedlt create "Null pointer exception" --profile bug
```

This creates a ticket using the settings defined in the `bug` profile (issue type, priority, labels, etc.).

### Multiple Profiles

You can combine multiple profiles, which are merged left-to-right (later profiles override earlier ones):

```bash
tedlt create "Critical production issue" --profile bug --profile urgent
```

The `urgent` profile settings will override any conflicting settings from the `bug` profile, while keeping non-conflicting fields from both.

## CLI Overrides

Override configuration values directly from the command line:

### Override Jira URL

```bash
tedlt create "New feature" --jira-url https://other-instance.atlassian.net
```

### Override Project Key

```bash
tedlt create "Update docs" --project-key DOCS
```

### Combine Overrides with Profiles

```bash
tedlt create "Fix bug in other project" --profile bug --project-key OTHER
```

## How Settings Are Merged

Settings are applied in the following priority order (highest to lowest):

1. **CLI overrides** (`--jira-url`, `--project-key`)
2. **Profile-specific settings** (from `--profile`)
3. **Default profile** (automatically applied to all tickets)
4. **Top-level configuration** (from config file)

### Example

Given this configuration:

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
      "project_key": "BUGS",
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug"]
      }
    }
  }
}
```

Running this command:

```bash
tedlt create "Fix issue" --profile bug --project-key OVERRIDE
```

Results in:
- **Jira URL**: `https://company.atlassian.net` (from top-level config)
- **Project Key**: `OVERRIDE` (from CLI override)
- **Issue Type**: `10004` (from bug profile)
- **Priority**: `2` (from bug profile, overrides default)
- **Labels**: `["auto-created", "bug"]` (arrays are concatenated)

## Verbose Output

Enable verbose logging to see detailed information about the ticket creation process:

```bash
tedlt create "Debug this" --profile bug --verbose
```

Useful for debugging configuration issues.

## Next Steps

- **[Using Profiles](/tedlt/usage/profiles/)** - Learn how to configure profiles
- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Build complex profiles
- **[Configuration Reference](/tedlt/reference/config-schema/)** - See all available options
