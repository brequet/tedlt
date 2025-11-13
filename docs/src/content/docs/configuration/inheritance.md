---
title: Profile Inheritance
description: Build complex profiles from simple building blocks using inheritance
---

Profile inheritance allows you to create reusable configuration profiles that extend and compose other profiles, reducing duplication and making your configuration more maintainable.

## What is Profile Inheritance?

Profile inheritance lets a profile inherit settings from one or more parent profiles. The child profile can override specific settings while keeping the rest from the parent.

This enables you to:
- Reduce duplication in configuration
- Build complex profiles from simple components
- Create hierarchies of related profiles
- Compose orthogonal concerns (e.g., issue type + priority level)

## Basic Syntax

Add an `inherits` field to your profile with an array of profile names:

```json
{
  "profiles": {
    "parent": {
      "jira_url": "https://example.atlassian.net",
      "project_key": "PARENT"
    },
    "child": {
      "project_key": "CHILD",
      "inherits": ["parent"]
    }
  }
}
```

When using the `child` profile, it inherits `jira_url` from `parent` but overrides `project_key`.

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
        "priority": { "id": "3" },
        "labels": ["base-label"]
      }
    },
    "dev": {
      "project_key": "DEV",
      "fields": {
        "labels": ["dev-label"]
      },
      "inherits": ["base"]
    }
  }
}
```

The `dev` profile will have:
- **jira_url**: `"https://company.atlassian.net"` (from base)
- **project_key**: `"DEV"` (overrides base)
- **priority**: `{ "id": "3" }` (from base)
- **labels**: `["base-label", "dev-label"]` (arrays are concatenated)

### Multiple Inheritance

A profile can inherit from multiple parents. They are applied **left-to-right**, with later profiles overriding earlier ones:

```json
{
  "profiles": {
    "base1": {
      "jira_url": "https://base1.atlassian.net",
      "project_key": "BASE1",
      "fields": { "field1": "from-base1" }
    },
    "base2": {
      "jira_url": "https://base2.atlassian.net",
      "fields": { "field2": "from-base2" }
    },
    "child": {
      "project_key": "CHILD",
      "inherits": ["base1", "base2"]
    }
  }
}
```

The `child` profile will have:
- **jira_url**: `"https://base2.atlassian.net"` (base2 overrides base1)
- **project_key**: `"CHILD"` (child overrides both)
- **field1**: `"from-base1"` (from base1)
- **field2**: `"from-base2"` (from base2)

### Recursive Inheritance

Inheritance works recursively—if a parent inherits from another profile, the entire chain is resolved:

```json
{
  "profiles": {
    "grandparent": {
      "jira_url": "https://example.atlassian.net",
      "fields": { "field1": "grandparent" }
    },
    "parent": {
      "project_key": "PARENT",
      "fields": { "field2": "parent" },
      "inherits": ["grandparent"]
    },
    "child": {
      "fields": { "field3": "child" },
      "inherits": ["parent"]
    }
  }
}
```

Resolution order: `grandparent` → `parent` → `child`

The `child` profile gets all three fields merged together.

## Priority Rules

Settings are merged with the following priority (highest to lowest):

1. **CLI overrides** (`--jira-url`, `--project-key`)
2. **Profile's own settings**
3. **Explicitly inherited profiles** (right-to-left in the inherits array)
4. **Default profile** (automatically applied)
5. **Top-level configuration**

### Merge Behavior

Different data types are merged differently:

#### Scalars (Strings, Numbers, Booleans)

Child value completely replaces parent value:

```json
{
  "profiles": {
    "parent": { "project_key": "PARENT" },
    "child": { 
      "project_key": "CHILD",
      "inherits": ["parent"]
    }
  }
}
```

Result: `"CHILD"` (replaces `"PARENT"`)

#### Arrays

Arrays are concatenated (parent elements + child elements):

```json
{
  "profiles": {
    "parent": {
      "fields": {
        "labels": ["parent-label"]
      }
    },
    "child": {
      "fields": {
        "labels": ["child-label"]
      },
      "inherits": ["parent"]
    }
  }
}
```

Result: `["parent-label", "child-label"]`

#### Objects

Objects are deep-merged recursively (nested fields are merged, not replaced):

```json
{
  "profiles": {
    "base": {
      "fields": {
        "customfield_1": {
          "property1": "base",
          "property2": "base"
        }
      }
    },
    "child": {
      "fields": {
        "customfield_1": {
          "property2": "child",
          "property3": "child"
        }
      },
      "inherits": ["base"]
    }
  }
}
```

Result:
```json
{
  "customfield_1": {
    "property1": "base",
    "property2": "child",
    "property3": "child"
  }
}
```

## The Default Profile

The `default` profile is special and has unique behavior.

### Automatic Inheritance

**All profiles automatically inherit from the `default` profile** as their lowest-priority base, even if they don't explicitly specify it in their `inherits` field.

```json
{
  "profiles": {
    "default": {
      "jira_url": "https://company.atlassian.net",
      "fields": {
        "priority": { "id": "3" }
      }
    },
    "my-profile": {
      "project_key": "MINE"
    }
  }
}
```

`my-profile` automatically gets `jira_url` and `priority` from `default`.

### Restrictions

**The `default` profile CANNOT have an `inherits` field.** This prevents circular dependencies and maintains a clear inheritance hierarchy.

```json
{
  "profiles": {
    "default": {
      "jira_url": "https://example.atlassian.net",
      "inherits": ["base"]
    }
  }
}
```

❌ This will cause an error!

## Practical Examples

### Bug Severity Hierarchy

```json
{
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
    "bug-low": {
      "inherits": ["bug-base"],
      "fields": {
        "priority": { "id": "4" }
      }
    },
    "bug-high": {
      "inherits": ["bug-base"],
      "fields": {
        "priority": { "id": "2" },
        "labels": ["urgent"]
      }
    },
    "bug-critical": {
      "inherits": ["bug-high"],
      "fields": {
        "priority": { "id": "1" },
        "labels": ["critical", "needs-immediate-action"]
      }
    }
  }
}
```

Usage:
```bash
tedlt create "Minor UI issue" --profile bug-low
tedlt create "Payment error" --profile bug-high
tedlt create "Data loss bug" --profile bug-critical
```

### Composition Pattern

Create small, focused profiles and compose them:

```json
{
  "profiles": {
    "with-assignee": {
      "fields": {
        "assignee": { "id": "USER123" }
      }
    },
    "with-watchers": {
      "fields": {
        "watchers": [{ "id": "USER456" }]
      }
    },
    "high-priority": {
      "fields": {
        "priority": { "id": "2" }
      }
    },
    "important-task": {
      "inherits": ["with-assignee", "with-watchers", "high-priority"],
      "fields": {
        "labels": ["important"]
      }
    }
  }
}
```

### Team-Based Profiles

```json
{
  "properties": {
    "frontend_team": "10100",
    "backend_team": "10101"
  },
  "profiles": {
    "default": {
      "jira_url": "https://company.atlassian.net",
      "project_key": "SHARED"
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
    "fullstack": {
      "inherits": ["frontend", "backend"],
      "fields": {
        "labels": ["fullstack"]
      }
    }
  }
}
```

The `fullstack` profile gets:
- Components from both frontend and backend (array concatenation)
- Labels: `["frontend", "backend", "fullstack"]`

## Error Handling

### Circular Dependencies

Circular inheritance is detected and will cause an error:

```json
{
  "profiles": {
    "a": { "inherits": ["b"] },
    "b": { "inherits": ["a"] }
  }
}
```

❌ Error: `Circular dependency detected in profile inheritance: a -> b -> a`

### Profile Not Found

Referencing a non-existent profile in `inherits`:

```json
{
  "profiles": {
    "child": {
      "inherits": ["nonexistent"]
    }
  }
}
```

❌ Error: `Profile not found: nonexistent`

### Self-Inheritance

A profile cannot inherit from itself:

```json
{
  "profiles": {
    "self": {
      "inherits": ["self"]
    }
  }
}
```

❌ Error: `Circular dependency detected in profile inheritance: self -> self`

## Best Practices

### Use the Default Profile for Common Settings

Put settings that apply to most issues in the `default` profile:

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

### Create Hierarchies

Build inheritance trees for related issue types:

```json
{
  "profiles": {
    "bug": { },
    "bug-high": { "inherits": ["bug"] },
    "bug-critical": { "inherits": ["bug-high"] }
  }
}
```

### Compose with Multiple Inheritance

Combine orthogonal concerns:

```json
{
  "profiles": {
    "bug": { "fields": { "issuetype": { "id": "10004" } } },
    "urgent": { "fields": { "priority": { "id": "1" } } },
    "security": { "fields": { "labels": ["security"] } }
  }
}

// Use: --profile bug --profile urgent --profile security
```

### Keep Profiles Focused

Each profile should represent one concern:

✅ Good:
```json
{
  "profiles": {
    "bug": { },
    "high-priority": { },
    "frontend": { }
  }
}
```

❌ Bad:
```json
{
  "profiles": {
    "bug-high-frontend": { }
  }
}
```

### Avoid Deep Nesting

More than 3-4 levels of inheritance can become hard to understand:

✅ Good: `default` → `bug` → `bug-critical`

❌ Bad: `default` → `base` → `bug` → `bug-high` → `bug-critical` → `bug-critical-security`

### Test Your Profiles

Use verbose mode to verify inheritance works as expected:

```bash
tedlt create "Test" --profile complex-profile --verbose
```

This shows the full inheritance chain and merged configuration.

## Usage

Use profiles with the `--profile` flag:

```bash
# Single profile
tedlt create "Fix bug" --profile bug

# Multiple profiles (merged left-to-right)
tedlt create "Critical fix" --profile bug --profile critical

# Override with CLI arguments (highest priority)
tedlt create "Task" --profile dev --project-key OVERRIDE
```

## Next Steps

- **[Using Profiles](/tedlt/usage/profiles/)** - Learn more about profile configuration
- **[Property Templates](/tedlt/configuration/properties/)** - Use variables in profiles
- **[Configuration Schema](/tedlt/reference/config-schema/)** - Complete reference