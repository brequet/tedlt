# Profile Inheritance

Profile inheritance allows you to create reusable configuration profiles that can extend and compose other profiles, reducing duplication and making your configuration more maintainable.

## Table of Contents

- [Basic Concepts](#basic-concepts)
- [How It Works](#how-it-works)
- [Priority Rules](#priority-rules)
- [The Default Profile](#the-default-profile)
- [Examples](#examples)
- [Advanced Patterns](#advanced-patterns)
- [Error Handling](#error-handling)

## Basic Concepts

### What is Profile Inheritance?

Profile inheritance allows a profile to inherit settings from one or more parent profiles. The child profile can override specific settings while keeping the rest from the parent.

### Syntax

Add an `inherits` field to your profile with an array of profile names:

```jsonc
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

## How It Works

### Simple Inheritance

When a profile inherits from another, all settings from the parent are merged into the child:

```jsonc
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
- `jira_url`: `"https://company.atlassian.net"` (from base)
- `project_key`: `"DEV"` (overrides base)
- `fields.priority`: `{ "id": "3" }` (from base)
- `fields.labels`: `["base-label", "dev-label"]` (arrays are concatenated)

### Multiple Inheritance

A profile can inherit from multiple parents. They are applied left-to-right, with later profiles overriding earlier ones:

```jsonc
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
- `jira_url`: `"https://base2.atlassian.net"` (base2 overrides base1)
- `project_key`: `"CHILD"` (child overrides both)
- `fields.field1`: `"from-base1"` (from base1)
- `fields.field2`: `"from-base2"` (from base2)

### Recursive Inheritance

Inheritance works recursively - if a parent inherits from another profile, the entire chain is resolved:

```jsonc
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

## Priority Rules

Settings are merged with the following priority (highest to lowest):

1. **CLI overrides** (passed via command line arguments)
2. **Profile's own settings**
3. **Explicitly inherited profiles** (right-to-left in the inherits array)
4. **Default profile** (if it exists)
5. **Top-level configuration**

### Merge Behavior

- **Scalars** (strings, numbers, booleans): Child value completely replaces parent value
- **Arrays**: Concatenated (parent elements + child elements)
- **Objects**: Deep-merged recursively (nested fields are merged, not replaced)

#### Example: Object Deep Merge

```jsonc
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

Result in `child`:
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

The `default` profile is special:

### Automatic Inheritance

**All profiles automatically inherit from the `default` profile** as their lowest-priority base, even if they don't explicitly specify it in their `inherits` field.

```jsonc
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
      // Automatically inherits from "default"
    }
  }
}
```

### Restrictions

**The `default` profile CANNOT have an `inherits` field.** This prevents circular dependencies and maintains a clear inheritance hierarchy.

```jsonc
{
  "profiles": {
    "default": {
      "jira_url": "https://example.atlassian.net",
      "inherits": ["base"]  // ❌ ERROR: Not allowed!
    }
  }
}
```

## Examples

### Example 1: Bug Severity Hierarchy

```jsonc
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
      "fields": {
        "priority": { "id": "4" }
      },
      "inherits": ["bug-base"]
    },
    "bug-high": {
      "fields": {
        "priority": { "id": "2" },
        "labels": ["urgent"]
      },
      "inherits": ["bug-base"]
    },
    "bug-critical": {
      "fields": {
        "priority": { "id": "1" },
        "labels": ["critical", "needs-immediate-action"]
      },
      "inherits": ["bug-high"]
    }
  }
}
```

### Example 2: Environment-Specific Profiles

```jsonc
{
  "profiles": {
    "default": {
      "jira_url": "https://company.atlassian.net",
      "fields": {
        "components": [{ "id": "10000" }]
      }
    },
    "base-feature": {
      "fields": {
        "issuetype": { "id": "10001" },
        "labels": ["feature"]
      }
    },
    "dev-feature": {
      "project_key": "DEV",
      "fields": {
        "labels": ["development"]
      },
      "inherits": ["base-feature"]
    },
    "prod-feature": {
      "project_key": "PROD",
      "fields": {
        "labels": ["production"],
        "priority": { "id": "2" }
      },
      "inherits": ["base-feature"]
    }
  }
}
```

### Example 3: Team-Based Profiles

```jsonc
{
  "properties": {
    "frontend_team_id": "10100",
    "backend_team_id": "10101"
  },
  "profiles": {
    "default": {
      "jira_url": "https://company.atlassian.net",
      "project_key": "SHARED"
    },
    "frontend": {
      "fields": {
        "components": [{ "id": "${frontend_team_id}" }],
        "labels": ["frontend"]
      }
    },
    "backend": {
      "fields": {
        "components": [{ "id": "${backend_team_id}" }],
        "labels": ["backend"]
      }
    },
    "fullstack": {
      "fields": {
        "labels": ["fullstack"]
      },
      "inherits": ["frontend", "backend"]
    }
  }
}
```

## Advanced Patterns

### Diamond Inheritance

When a profile inherits from multiple profiles that share a common ancestor, the common ancestor is only included once in the resolution chain:

```
       default
       /    \
    base1  base2
       \    /
       child
```

```jsonc
{
  "profiles": {
    "default": {
      "fields": { "f1": "default" }
    },
    "base1": {
      "fields": { "f2": "base1" },
      "inherits": []  // Still inherits from default automatically
    },
    "base2": {
      "fields": { "f3": "base2" },
      "inherits": []  // Still inherits from default automatically
    },
    "child": {
      "inherits": ["base1", "base2"]
    }
  }
}
```

Resolution order: `default` → `base1` → `base2` → `child`

The `default` profile only appears once, even though it's reached through both `base1` and `base2`.

### Composition Pattern

Create small, focused profiles and compose them:

```jsonc
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
    "with-due-date": {
      "fields": {
        "duedate": "2024-12-31"
      }
    },
    "important-task": {
      "fields": {
        "priority": { "id": "1" }
      },
      "inherits": ["with-assignee", "with-watchers", "with-due-date"]
    }
  }
}
```

## Error Handling

### Circular Dependencies

Circular inheritance is detected and will cause an error:

```jsonc
{
  "profiles": {
    "a": { "inherits": ["b"] },
    "b": { "inherits": ["a"] }  // ❌ Circular dependency!
  }
}
```

Error: `Circular dependency detected in profile inheritance: a -> b -> a`

### Profile Not Found

Referencing a non-existent profile in `inherits` will cause an error:

```jsonc
{
  "profiles": {
    "child": {
      "inherits": ["nonexistent"]  // ❌ Profile doesn't exist!
    }
  }
}
```

Error: `Profile not found: nonexistent`

### Self-Inheritance

A profile cannot inherit from itself:

```jsonc
{
  "profiles": {
    "self": {
      "inherits": ["self"]  // ❌ Cannot inherit from self!
    }
  }
}
```

Error: `Circular dependency detected in profile inheritance: self -> self`

## Best Practices

1. **Use the default profile for common settings**: Put settings that apply to most issues in the `default` profile.

2. **Create hierarchies**: Build inheritance trees for related issue types (e.g., `bug` → `bug-critical`).

3. **Compose with multiple inheritance**: Combine orthogonal concerns (e.g., issue type + priority level).

4. **Keep profiles focused**: Each profile should represent one concern or dimension of configuration.

5. **Document your inheritance structure**: Use comments to explain complex inheritance relationships.

6. **Test your profiles**: Use different profile combinations to ensure they merge as expected.

7. **Avoid deep nesting**: More than 3-4 levels of inheritance can become hard to understand.

8. **Be explicit**: Even though `default` is automatic, document when you're relying on it.

## Usage

Use profiles with the `--profile` flag:

```bash
# Use a single profile
tedlt create "Fix bug" --profile bug

# Use multiple profiles (merged left-to-right)
tedlt create "Critical fix" --profile bug --profile critical

# Override with CLI arguments (highest priority)
tedlt create "Task" --profile dev --project-key OVERRIDE
```

## Summary

Profile inheritance provides:
- ✅ Reduced duplication in configuration
- ✅ Better organization of related profiles
- ✅ Flexible composition with multiple inheritance
- ✅ Automatic merging with the `default` profile
- ✅ Deep merging of nested objects
- ✅ Array concatenation for combining lists
- ✅ Circular dependency detection
- ✅ Clear priority rules

See the [example configuration](../examples/config-with-inheritance.jsonc) for a complete working example.
