---
title: Property Templates
description: Use variables to reuse values across your configuration
---

Property templates allow you to define reusable variables that can be referenced throughout your configuration. This reduces duplication and makes your configuration easier to maintain.

## What Are Property Templates?

Properties are named values that you define once and reference multiple times using the `${property_name}` syntax.

## Basic Syntax

### Defining Properties

Properties are defined in the top-level `properties` object, nested properties are supported:

```json
{
  "properties": {
    "property_name": "value",
    "another_property": "another value",
    "issueTypes": {
      "bug": "10004",
      "story": "10002"
    }
  }
}
```

### Referencing Properties

Reference properties using `${property_name}` (or for nested properties, join with `.`):

```json
{
  "properties": {
    "team_lead": "USER123",
    "issueTypes": {
      "bug": "10004",
      "story": "10002"
    }
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "${team_lead}" },
        "issuetype": {"id": "${issueTypes.bug}"}
      }
    }
  }
}
```

When creating a ticket with the `bug` profile, `${team_lead}` is replaced with `USER123`.

## Advanced Usage

### Multiple Properties in One Value

Combine multiple properties in a single value:

```json
{
  "properties": {
    "project": "MYPROJ",
    "epic_number": "100"
  },
  "profiles": {
    "story": {
      "fields": {
        "customfield_10050": "${project}-${epic_number}"
      }
    }
  }
}
```

Result: `"MYPROJ-100"`

## Finding Property Values

Use `tedlt info` commands to discover the values you need.

cf [Info Commands](/tedlt/usage/info-commands/).

## Next Steps

**Usage Guides:**
- [Profiles](/tedlt/configuration/profiles/) - Use properties in profiles
- [Profile Inheritance](/tedlt/configuration/inheritance/) - Combine properties with inheritance

**Reference:**
- [Configuration Schema](/tedlt/reference/config-schema/) - Complete syntax reference
