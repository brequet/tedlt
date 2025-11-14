---
title: Property Templates
description: Use variables to reuse values across your configuration
---

Property templates allow you to define reusable variables that can be referenced throughout your configuration. This reduces duplication and makes your configuration easier to maintain.

## What Are Property Templates?

Properties are named values that you define once and reference multiple times using the `${property_name}` syntax.

Instead of repeating the same value in multiple places:

```json
{
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "USER123" }
      }
    },
    "task": {
      "fields": {
        "assignee": { "id": "USER123" }
      }
    }
  }
}
```

You can define it once as a property:

```json
{
  "properties": {
    "team_lead": "USER123"
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "${team_lead}" }
      }
    },
    "task": {
      "fields": {
        "assignee": { "id": "${team_lead}" }
      }
    }
  }
}
```

## Basic Syntax

### Defining Properties

Properties are defined in the top-level `properties` object:

```json
{
  "properties": {
    "property_name": "value",
    "another_property": "another value"
  }
}
```

Property names can contain:
- Letters (a-z, A-Z)
- Numbers (0-9)
- Underscores (_)
- Hyphens (-)

### Referencing Properties

Reference properties using double curly braces:

```json
{
  "properties": {
    "team_lead": "USER123"
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "${team_lead}" }
      }
    }
  }
}
```

When creating a ticket with the `bug` profile, `${team_lead}` is replaced with `USER123`.

## Common Use Cases

### User IDs

Store frequently used user IDs:

```json
{
  "properties": {
    "team_lead": "USER123",
    "qa_lead": "USER456",
    "product_owner": "USER789"
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "${team_lead}" }
      }
    },
    "qa-task": {
      "fields": {
        "assignee": { "id": "${qa_lead}" }
      }
    }
  }
}
```

### Component IDs

Store team or area component IDs:

```json
{
  "properties": {
    "frontend_component": "10100",
    "backend_component": "10101",
    "mobile_component": "10102"
  },
  "profiles": {
    "frontend": {
      "fields": {
        "components": [{ "id": "${frontend_component}" }]
      }
    },
    "backend": {
      "fields": {
        "components": [{ "id": "${backend_component}" }]
      }
    }
  }
}
```

### Priority Levels

Define standard priority levels:

```json
{
  "properties": {
    "priority_low": "5",
    "priority_normal": "3",
    "priority_high": "2",
    "priority_critical": "1"
  },
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "${priority_normal}" }
      }
    },
    "urgent": {
      "fields": {
        "priority": { "id": "${priority_critical}" }
      }
    }
  }
}
```

### Epic Links

Store epic keys for linking stories:

```json
{
  "properties": {
    "auth_epic": "PROJ-100",
    "api_epic": "PROJ-101",
    "ui_epic": "PROJ-102"
  },
  "profiles": {
    "auth-story": {
      "fields": {
        "customfield_10050": "${auth_epic}"
      }
    },
    "api-story": {
      "fields": {
        "customfield_10050": "${api_epic}"
      }
    }
  }
}
```

### Custom Field Values

Store complex custom field values:

```json
{
  "properties": {
    "sprint_current": "Sprint 23",
    "team_name": "Platform Team",
    "environment_prod": "production"
  },
  "profiles": {
    "default": {
      "fields": {
        "customfield_10060": "${sprint_current}",
        "customfield_10061": { "value": "${team_name}" }
      }
    }
  }
}
```

## Advanced Usage

### Multiple Properties in One Field

You can use multiple properties in a single value:

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

### Properties in Arrays

Use properties in array elements:

```json
{
  "properties": {
    "frontend_component": "10100",
    "backend_component": "10101"
  },
  "profiles": {
    "fullstack": {
      "fields": {
        "components": [
          { "id": "${frontend_component}" },
          { "id": "${backend_component}" }
        ]
      }
    }
  }
}
```

### Properties with Inheritance

Properties work seamlessly with profile inheritance:

```json
{
  "properties": {
    "default_priority": "3",
    "urgent_priority": "1"
  },
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "${default_priority}" }
      }
    },
    "urgent": {
      "inherits": ["default"],
      "fields": {
        "priority": { "id": "${urgent_priority}" }
      }
    }
  }
}
```

## Complete Example

```json
{
  "jira_url": "https://company.atlassian.net",
  "project_key": "PROJ",
  
  "properties": {
    "team_lead": "USER123",
    "qa_lead": "USER456",
    "frontend_component": "10100",
    "backend_component": "10101",
    "priority_normal": "3",
    "priority_high": "2",
    "priority_critical": "1",
    "auth_epic": "PROJ-100"
  },
  
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "${priority_normal}" },
        "labels": ["auto-created"]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "${priority_high}" },
        "assignee": { "id": "${team_lead}" },
        "labels": ["bug"]
      }
    },
    "frontend-story": {
      "fields": {
        "issuetype": { "id": "10002" },
        "components": [{ "id": "${frontend_component}" }],
        "customfield_10050": "${auth_epic}",
        "labels": ["frontend", "story"]
      }
    },
    "critical-bug": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "${priority_critical}" },
        "assignee": { "id": "${team_lead}" },
        "labels": ["critical", "urgent"]
      }
    }
  }
}
```

## How Property Resolution Works

When tedlt processes your configuration:

1. **Loading**: Properties are loaded from the `properties` object
2. **Resolution**: All `${property_name}` references are replaced with their values
3. **Merging**: Profiles are merged (after property resolution)
4. **Validation**: The final configuration is validated

### Resolution Example

Before resolution:
```json
{
  "properties": {
    "user": "USER123"
  },
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "${user}" }
      }
    }
  }
}
```

After resolution:
```json
{
  "profiles": {
    "bug": {
      "fields": {
        "assignee": { "id": "USER123" }
      }
    }
  }
}
```

## Best Practices

### Use Descriptive Names

Choose clear, descriptive property names:

✅ Good:
```json
{
  "properties": {
    "frontend_team_lead": "USER123",
    "default_priority_level": "3"
  }
}
```

❌ Bad:
```json
{
  "properties": {
    "user1": "USER123",
    "p": "3"
  }
}
```

### Group Related Properties

Organize properties by category using naming conventions:

```json
{
  "properties": {
    "user_team_lead": "USER123",
    "user_qa_lead": "USER456",
    "component_frontend": "10100",
    "component_backend": "10101",
    "priority_low": "5",
    "priority_high": "2"
  }
}
```

### Document Your Properties

Use comments (in JSONC files) to document property purposes:

```jsonc
{
  "properties": {
    // User IDs for automatic assignment
    "team_lead": "USER123",
    
    // Component IDs from project info
    "frontend_component": "10100",  // Frontend team component
    "backend_component": "10101",   // Backend team component
    
    // Standard priority levels
    "priority_normal": "3",   // Default for most tasks
    "priority_high": "2"      // For urgent issues
  }
}
```

### Keep Properties Simple

Properties should be simple string values. Avoid complex nested structures:

✅ Good:
```json
{
  "properties": {
    "user_id": "USER123",
    "component_id": "10100"
  }
}
```

❌ Bad:
```json
{
  "properties": {
    "user_object": { "id": "USER123", "name": "John" }
  }
}
```

### Update Properties When Things Change

When IDs or values change in Jira, update properties in one place:

```json
{
  "properties": {
    "team_lead": "USER789"  // Changed from USER123
  }
}
```

All profiles using `${team_lead}` automatically get the new value.

## Finding Property Values

Use `tedlt info` commands to discover the values you need:

### Find User IDs

```bash
# View a ticket with the user assigned
tedlt info ticket PROJ-123
# Look for the assignee.id field
```

### Find Component IDs

```bash
tedlt info project PROJ
# Look in the components array
```

### Find Custom Field Values

```bash
tedlt info fields --project-key PROJ --issue-type 10001
# Find the field and its possible values
```

### Find Epic Keys

```bash
tedlt info epics --project-key PROJ
# Get the epic keys
```

## Troubleshooting

### "Property not found" Error

**Error:** `Property 'team_lead' referenced but not defined`

**Cause:** A profile references a property that doesn't exist in the `properties` object.

**Solution:** Add the property to your configuration:
```json
{
  "properties": {
    "team_lead": "USER123"
  }
}
```

### Property Not Being Replaced

**Issue:** You see `${property_name}` in the created ticket instead of the value.

**Cause:** Property substitution happens during configuration loading. If you see the literal `${...}`, the property wasn't found.

**Solution:** 
1. Check property spelling
2. Ensure the property exists in the `properties` object
3. Verify you're using the correct syntax: `${property_name}`

### Testing Property Resolution

Use verbose mode to see resolved values:

```bash
tedlt create "Test" --profile bug --verbose
```

This shows the configuration after property resolution, so you can verify values are correct.

## Limitations

### No Nested Properties

You cannot reference properties within property values:

❌ Not supported:
```json
{
  "properties": {
    "base": "USER",
    "user_id": "${base}123"
  }
}
```

### No Computed Properties

Properties must be static values, not computed or dynamic:

❌ Not supported:
```json
{
  "properties": {
    "today": "${date}"
  }
}
```

### Properties Are Strings

All property values are treated as strings. They're replaced verbatim wherever referenced.

## Next Steps

- **[Using Profiles](/tedlt/usage/profiles/)** - Use properties in profiles
- **[Profile Inheritance](/tedlt/configuration/inheritance/)** - Combine with inheritance
- **[Configuration Schema](/tedlt/reference/config-schema/)** - Complete reference
