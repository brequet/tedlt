---
title: Configuration Schema
description: Complete reference for the tedlt configuration file schema
---

This page provides a complete reference for all fields available in the tedlt configuration file.

## Configuration File Structure

```json
{
  "jira_url": "string",
  "project_key": "string",
  "properties": {
    "property_name": "value"
  },
  "profiles": {
    "profile_name": {
      "jira_url": "string",
      "project_key": "string",
      "inherits": ["profile1", "profile2"],
      "fields": {
        "field_name": "value"
      }
    }
  }
}
```

## Root Level Fields

### `jira_url`

**Type:** String  
**Required:** No (but recommended)  
**Description:** The base URL of your Jira instance.

```json
{
  "jira_url": "https://yourcompany.atlassian.net"
}
```

**Valid formats:**
- `https://yourcompany.atlassian.net`
- `https://jira.yourcompany.com`

**Notes:**
- Must include `https://`
- Trailing slash is optional
- Can be overridden per-profile or via `--jira-url` flag

---

### `project_key`

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
- Examples: `PROJ`, `KAN`, `DEV`, `TEAM`

**Notes:**
- Can be overridden per-profile or via `--project-key` flag

---

### `properties`

**Type:** Object  
**Required:** No  
**Description:** Reusable variables for use in profiles.

```json
{
  "properties": {
    "team_lead": "USER123",
    "default_priority": "3",
    "frontend_component": "10100"
  }
}
```

**Keys:**
- Can contain letters, numbers, underscores, hyphens
- Case-sensitive

**Values:**
- Must be strings
- Referenced in profiles using `{{property_name}}` syntax

**Example usage:**
```json
{
  "properties": {
    "user_id": "USER123"
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "{{user_id}}" }
      }
    }
  }
}
```

---

### `profiles`

**Type:** Object  
**Required:** No  
**Description:** Named configurations for different ticket types.

```json
{
  "profiles": {
    "bug": { },
    "feature": { },
    "default": { }
  }
}
```

**Keys:**
- Profile names
- Case-sensitive
- Should be descriptive (e.g., `bug`, `feature-urgent`, `frontend-task`)

**Values:**
- Profile objects (see [Profile Schema](#profile-schema))

---

## Profile Schema

Each profile in the `profiles` object can contain the following fields:

### `jira_url` (in profile)

**Type:** String  
**Required:** No  
**Description:** Override the Jira URL for this profile.

```json
{
  "profiles": {
    "personal": {
      "jira_url": "https://personal.atlassian.net"
    }
  }
}
```

**Use case:** Working with multiple Jira instances.

---

### `project_key` (in profile)

**Type:** String  
**Required:** No  
**Description:** Override the project key for this profile.

```json
{
  "profiles": {
    "backend": {
      "project_key": "BACKEND"
    }
  }
}
```

**Use case:** Creating tickets in different projects.

---

### `inherits`

**Type:** Array of strings  
**Required:** No  
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

**Rules:**
- Profiles are merged left-to-right
- Can inherit from multiple profiles
- Inheritance is recursive
- Circular dependencies are not allowed
- The `default` profile cannot have an `inherits` field

**Merge behavior:**
- Scalars: Child overrides parent
- Arrays: Concatenated
- Objects: Deep-merged

---

### `fields`

**Type:** Object  
**Required:** No  
**Description:** Jira field values to include when creating tickets.

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

**Keys:**
- Jira field names (e.g., `issuetype`, `priority`, `customfield_10050`)

**Values:**
- Depends on field type (see [Field Types](#field-types))

---

## Field Types

Different Jira fields require different value formats.

### Issue Type

**Field name:** `issuetype`  
**Format:** Object with `id` property

```json
{
  "fields": {
    "issuetype": { "id": "10004" }
  }
}
```

**How to find:**
```bash
tedlt info project PROJ
# Look in issueTypes array
```

**Common values:**
- `10001`: Task
- `10002`: Story
- `10003`: Epic
- `10004`: Bug

---

### Priority

**Field name:** `priority`  
**Format:** Object with `id` property

```json
{
  "fields": {
    "priority": { "id": "2" }
  }
}
```

**Standard values:**
- `1`: Highest
- `2`: High
- `3`: Medium
- `4`: Low
- `5`: Lowest

---

### Labels

**Field name:** `labels`  
**Format:** Array of strings

```json
{
  "fields": {
    "labels": ["bug", "critical", "frontend"]
  }
}
```

**Notes:**
- Arrays are concatenated when profiles are merged
- Labels are case-sensitive in Jira

---

### Components

**Field name:** `components`  
**Format:** Array of objects with `id` property

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

**How to find:**
```bash
tedlt info project PROJ
# Look in components array
```

---

### Assignee

**Field name:** `assignee`  
**Format:** Object with `id` property

```json
{
  "fields": {
    "assignee": { "id": "USER123" }
  }
}
```

**How to find user IDs:**
```bash
tedlt info ticket PROJ-123
# Look at assignee field in an existing ticket
```

---

### Reporter

**Field name:** `reporter`  
**Format:** Object with `id` property

```json
{
  "fields": {
    "reporter": { "id": "USER456" }
  }
}
```

---

### Description

**Field name:** `description`  
**Format:** String or Atlassian Document Format (ADF) object

**Simple text:**
```json
{
  "fields": {
    "description": "This is a bug description"
  }
}
```

**ADF format:**
```json
{
  "fields": {
    "description": {
      "type": "doc",
      "version": 1,
      "content": [
        {
          "type": "paragraph",
          "content": [
            {
              "type": "text",
              "text": "This is a description"
            }
          ]
        }
      ]
    }
  }
}
```

---

### Due Date

**Field name:** `duedate`  
**Format:** String in `YYYY-MM-DD` format

```json
{
  "fields": {
    "duedate": "2024-12-31"
  }
}
```

---

### Fix Versions

**Field name:** `fixVersions`  
**Format:** Array of objects with `id` property

```json
{
  "fields": {
    "fixVersions": [
      { "id": "10200" }
    ]
  }
}
```

**How to find:**
```bash
tedlt info project PROJ
# Look in versions array
```

---

### Affects Versions

**Field name:** `versions`  
**Format:** Array of objects with `id` property

```json
{
  "fields": {
    "versions": [
      { "id": "10200" }
    ]
  }
}
```

---

### Epic Link

**Field name:** Varies by Jira instance (usually `customfield_10050`)  
**Format:** String (epic key)

```json
{
  "fields": {
    "customfield_10050": "PROJ-100"
  }
}
```

**How to find the field name:**
```bash
tedlt info fields --project-key PROJ --issue-type 10002
# Look for "Epic Link" field
```

**How to find epic keys:**
```bash
tedlt info epics --project-key PROJ
```

---

### Custom Fields

**Field name:** `customfield_XXXXX`  
**Format:** Varies by field type

**Text field:**
```json
{
  "fields": {
    "customfield_10060": "text value"
  }
}
```

**Select field (single):**
```json
{
  "fields": {
    "customfield_10061": { "id": "10300" }
  }
}
```

**Select field (with value):**
```json
{
  "fields": {
    "customfield_10062": { "value": "Option 1" }
  }
}
```

**Multi-select field:**
```json
{
  "fields": {
    "customfield_10063": [
      { "value": "Option 1" },
      { "value": "Option 2" }
    ]
  }
}
```

**Number field:**
```json
{
  "fields": {
    "customfield_10064": 42
  }
}
```

**Date field:**
```json
{
  "fields": {
    "customfield_10065": "2024-12-31"
  }
}
```

**User picker field:**
```json
{
  "fields": {
    "customfield_10066": { "id": "USER123" }
  }
}
```

**How to discover custom fields:**
```bash
tedlt info fields --project-key PROJ --issue-type 10001
```

---

## Special Profile: `default`

The `default` profile has special behavior:

### Automatic Inheritance

All profiles automatically inherit from `default` (if it exists), even without specifying it in `inherits`.

```json
{
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" }
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

Using `--profile bug` will include both `priority` from `default` and `issuetype` from `bug`.

### Restrictions

- The `default` profile **cannot** have an `inherits` field
- This prevents circular dependencies

---

## Priority and Merging

Settings are applied in this order (highest to lowest priority):

1. **CLI overrides** (`--jira-url`, `--project-key`)
2. **Profile's own settings**
3. **Explicitly inherited profiles** (right-to-left in inherits array)
4. **Default profile** (automatically applied)
5. **Top-level configuration**

### Merge Rules

**Scalars (strings, numbers, booleans):**
- Child completely replaces parent

**Arrays:**
- Concatenated (parent + child)

**Objects:**
- Deep-merged recursively

### Examples

**Scalar override:**
```json
{
  "project_key": "MAIN",
  "profiles": {
    "other": {
      "project_key": "OTHER"  // Replaces "MAIN"
    }
  }
}
```

**Array concatenation:**
```json
{
  "profiles": {
    "default": {
      "fields": { "labels": ["auto"] }
    },
    "bug": {
      "fields": { "labels": ["bug"] }
    }
  }
}
// Result: ["auto", "bug"]
```

**Object merge:**
```json
{
  "profiles": {
    "base": {
      "fields": {
        "customfield_1": {
          "prop1": "base",
          "prop2": "base"
        }
      }
    },
    "child": {
      "inherits": ["base"],
      "fields": {
        "customfield_1": {
          "prop2": "child",
          "prop3": "child"
        }
      }
    }
  }
}
// Result: { "prop1": "base", "prop2": "child", "prop3": "child" }
```

---

## Property Template Syntax

Properties use the `{{property_name}}` syntax.

### Definition

```json
{
  "properties": {
    "team_lead": "USER123",
    "component": "10100"
  }
}
```

### Usage

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "{{team_lead}}" },
        "components": [{ "id": "{{component}}" }]
      }
    }
  }
}
```

### Multiple Properties

```json
{
  "properties": {
    "project": "PROJ",
    "epic_num": "100"
  },
  "profiles": {
    "story": {
      "fields": {
        "customfield_10050": "{{project}}-{{epic_num}}"
      }
    }
  }
}
// Result: "PROJ-100"
```

---

## Validation Rules

tedlt validates your configuration and will report errors for:

### Invalid JSON Syntax

```
Error: Failed to parse config file: expected `,` at line 5
```

Fix: Check JSON syntax (missing commas, quotes, brackets)

### Profile Not Found

```
Error: Profile 'nonexistent' referenced in inherits but not defined
```

Fix: Ensure all profiles in `inherits` arrays exist

### Circular Dependency

```
Error: Circular dependency detected in profile inheritance: a -> b -> a
```

Fix: Remove circular inheritance chains

### Default Profile Inheritance

```
Error: Default profile cannot have 'inherits' field
```

Fix: Remove `inherits` from the `default` profile

### Property Not Found

```
Error: Property 'team_lead' referenced but not defined
```

Fix: Add the property to the `properties` object

---

## Complete Examples

### Minimal Configuration

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ"
}
```

### Basic with Profiles

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
        "priority": { "id": "2" },
        "labels": ["bug"]
      }
    }
  }
}
```

### With Properties

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ",
  "properties": {
    "team_lead": "USER123",
    "frontend_component": "10100"
  },
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "3" }
      }
    },
    "frontend": {
      "fields": {
        "components": [{ "id": "{{frontend_component}}" }],
        "assignee": { "id": "{{team_lead}}" }
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
        "labels": ["critical", "urgent"]
      }
    }
  }
}
```

### Multi-Instance

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

---

## Next Steps

- **[Configuration Overview](/tedlt/configuration/overview/)** - Learn about configuration concepts
- **[Profiles](/tedlt/configuration/profiles/)** - Learn how to use profiles
- **[Property Templates](/tedlt/configuration/properties/)** - Learn about property templates
- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Learn about inheritance