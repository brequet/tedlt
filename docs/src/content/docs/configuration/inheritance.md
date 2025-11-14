---
title: Profile Inheritance
description: Build complex profiles from simple building blocks using inheritance
---

Profile inheritance allows you to create reusable configuration profiles that extend and compose other profiles, reducing duplication and making your configuration more maintainable.

## What is Profile Inheritance?

Profile inheritance lets a profile inherit settings from one or more parent profiles. The child profile can override specific settings while keeping the rest from the parent.

**Benefits:**
- Reduce duplication in configuration
- Build complex profiles from simple components
- Create hierarchies of related profiles
- Compose orthogonal concerns (e.g., issue type + priority level)

## Basic Syntax

Add an `inherits` field to your profile with an array of profile names:

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
        "labels": ["critical"]
      }
    }
  }
}
```

When using `bug-critical`, it inherits `issuetype` from `bug` but overrides `priority` and adds `labels`.

## How It Works

### Simple Inheritance

All settings from the parent are merged into the child:

```json
{
  "profiles": {
    "base": {
      "jira_url": "https://company.atlassian.net",
      "project_key": "BASE",
      "fields": {
        "priority": { "id": "3" }
      }
    },
    "dev": {
      "inherits": ["base"],
      "project_key": "DEV"
    }
  }
}
```

The `dev` profile gets:
- **jira_url**: `"https://company.atlassian.net"` (from base)
- **project_key**: `"DEV"` (overrides base)
- **priority**: `{ "id": "3" }` (from base)

### Multiple Inheritance

A profile can inherit from multiple parents, applied **left-to-right**:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" }
      }
    },
    "urgent": {
      "fields": {
        "priority": { "id": "1" }
      }
    },
    "security": {
      "fields": {
        "labels": ["security"]
      }
    },
    "critical-security-bug": {
      "inherits": ["bug", "urgent", "security"]
    }
  }
}
```

The `critical-security-bug` profile gets all fields from all three parent profiles.

### Recursive Inheritance

Inheritance works recursively—if a parent inherits from another profile, the entire chain is resolved:

```json
{
  "profiles": {
    "base": {
      "fields": {
        "priority": { "id": "3" }
      }
    },
    "bug": {
      "inherits": ["base"],
      "fields": {
        "issuetype": { "id": "10004" }
      }
    },
    "bug-critical": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "1" }
      }
    }
  }
}
```

Resolution order: `base` → `bug` → `bug-critical`

## Merge Behavior

Different data types merge differently. See the [Configuration Schema](/tedlt/reference/config-schema/) for complete merge rules.

**Quick summary:**
- **Scalars** (strings, numbers) - Child replaces parent
- **Arrays** - Concatenated (parent elements + child elements)
- **Objects** - Deep-merged recursively

**Example:**

```json
{
  "profiles": {
    "base": {
      "fields": {
        "labels": ["base"],
        "priority": { "id": "3" }
      }
    },
    "child": {
      "inherits": ["base"],
      "fields": {
        "labels": ["child"],
        "priority": { "id": "1" }
      }
    }
  }
}
```

Result:
- **labels**: `["base", "child"]` (concatenated)
- **priority**: `{ "id": "1" }` (replaced)

## The Default Profile

The `default` profile has special behavior.

### Automatic Inheritance

**All profiles automatically inherit from `default`** as their lowest-priority base, even without specifying it in `inherits`.

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

Using `--profile bug` automatically includes fields from `default`.

### Restrictions

**The `default` profile CANNOT have an `inherits` field.** This prevents circular dependencies.

## Configuration Priority

Settings are applied in this order (highest to lowest priority):

1. **CLI overrides** (`--jira-url`, `--project-key`)
2. **Profile's own settings**
3. **Explicitly inherited profiles** (right-to-left in inherits array)
4. **Default profile** (automatically applied)
5. **Top-level configuration**

See the [Configuration Schema](/tedlt/reference/config-schema/) for detailed priority rules.

## Usage

### Single Profile

```bash
tedlt create "Fix bug" --profile bug
```

### Multiple Profiles

```bash
# Profiles merged left-to-right
tedlt create "Critical fix" --profile bug --profile critical
```

### With CLI Overrides

```bash
# CLI arguments have highest priority
tedlt create "Task" --profile dev --project-key OVERRIDE
```

## Next Steps

**Related Configuration:**
- [Profiles](/tedlt/configuration/profiles/) - Learn about profile structure and usage
- [Property Templates](/tedlt/configuration/properties/) - Use variables with inheritance

**Reference:**
- [Configuration Schema](/tedlt/reference/config-schema/) - Complete merge rules and priority documentation
