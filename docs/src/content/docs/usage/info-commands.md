---
title: Discovering Jira Info
description: Learn how to query Jira metadata using info commands
---

The `info` command lets you discover Jira metadata like projects, boards, epics, fields, and tickets. This is essential for finding the IDs and values you need to configure profiles.

## Why Use Info Commands?

When configuring tedlt, you need specific IDs for:
- Issue types (Task, Bug, Story, etc.)
- Components (Frontend, Backend, etc.)
- Custom fields
- Epics
- Boards

Instead of searching through Jira's web interface, use `tedlt info` to quickly find this information.

## Project Information

Get detailed information about a project:

```bash
tedlt info project PROJ
```

### What You'll See

- **Project metadata**: Name, key, description
- **Issue types**: Available types and their IDs
- **Components**: Team components and their IDs
- **Versions**: Release versions and their IDs
- **Other project settings**

### Example Output

```json
{
  "key": "PROJ",
  "name": "My Project",
  "issueTypes": [
    { "id": "10001", "name": "Task" },
    { "id": "10004", "name": "Bug" },
    { "id": "10002", "name": "Story" },
    { "id": "10003", "name": "Epic" }
  ],
  "components": [
    { "id": "10100", "name": "Frontend" },
    { "id": "10101", "name": "Backend" },
    { "id": "10102", "name": "DevOps" }
  ],
  "versions": [
    { "id": "10200", "name": "v1.0.0" },
    { "id": "10201", "name": "v1.1.0" }
  ]
}
```

### Using with Profiles

When using profiles to set context:

```bash
tedlt info project --profile work
```

This uses the project key from the `work` profile.

### Overriding Project Key

Query a different project:

```bash
tedlt info project OTHER
```

Or use a CLI override:

```bash
tedlt info project --project-key OTHER
```

## Field Information

Discover available fields for a specific issue type:

```bash
tedlt info fields --project-key PROJ --issue-type 10001
```

### What You'll See

- **Field name**: The field identifier (e.g., `customfield_10050`)
- **Display name**: Human-readable name
- **Field type**: String, number, select, multi-select, etc.
- **Required**: Whether the field is mandatory
- **Allowed values**: For select fields, the available options

### Example Output

```json
{
  "fields": [
    {
      "key": "summary",
      "name": "Summary",
      "required": true,
      "schema": { "type": "string" }
    },
    {
      "key": "issuetype",
      "name": "Issue Type",
      "required": true,
      "schema": { "type": "issuetype" }
    },
    {
      "key": "priority",
      "name": "Priority",
      "required": false,
      "schema": { "type": "priority" },
      "allowedValues": [
        { "id": "1", "name": "Highest" },
        { "id": "2", "name": "High" },
        { "id": "3", "name": "Medium" },
        { "id": "4", "name": "Low" },
        { "id": "5", "name": "Lowest" }
      ]
    },
    {
      "key": "customfield_10050",
      "name": "Epic Link",
      "required": false,
      "schema": { "type": "string" }
    }
  ]
}
```

### Use Cases

1. **Find required fields** for a specific issue type
2. **Discover custom field names** for your configuration
3. **See available options** for select fields

## Ticket Information

Inspect an existing ticket to see its raw data:

```bash
tedlt info ticket PROJ-123
```

### What You'll See

The complete JSON representation of the ticket, including:
- All field values
- Custom field data
- Issue links
- Comments metadata
- Workflow status

### Example Output

```json
{
  "key": "PROJ-123",
  "fields": {
    "summary": "Fix login page error",
    "issuetype": {
      "id": "10004",
      "name": "Bug"
    },
    "priority": {
      "id": "2",
      "name": "High"
    },
    "labels": ["bug", "frontend"],
    "components": [
      { "id": "10100", "name": "Frontend" }
    ],
    "customfield_10050": "PROJ-100",
    "status": {
      "name": "In Progress"
    }
  }
}
```

### Use Cases

1. **Reverse-engineer field configurations**: Copy field values into your profiles
2. **Find custom field IDs**: See which `customfield_*` corresponds to which field
3. **Debug ticket creation**: Compare your configuration with working tickets

### Copying to Profile

When you find a ticket configured the way you want, copy its field values:

```json
{
  "profiles": {
    "like-proj-123": {
      "fields": {
        "issuetype": { "id": "10004" },
        "priority": { "id": "2" },
        "labels": ["bug", "frontend"],
        "components": [{ "id": "10100" }]
      }
    }
  }
}
```

## Board Information

List all available boards:

```bash
tedlt info boards
```

### Filter by Project

```bash
tedlt info boards --project PROJ
```

### What You'll See

```json
{
  "boards": [
    {
      "id": 1,
      "name": "PROJ Board",
      "type": "scrum"
    },
    {
      "id": 2,
      "name": "Team Kanban",
      "type": "kanban"
    }
  ]
}
```

### Use Cases

1. **Find board IDs** for epic queries
2. **See available boards** in your Jira instance
3. **Identify board types** (Scrum vs Kanban)

## Epic Information

List epics in a project:

```bash
tedlt info epics --project-key PROJ
```

### List Epics by Board

```bash
tedlt info epics --board-id 1
```

### What You'll See

```json
{
  "epics": [
    {
      "key": "PROJ-100",
      "name": "User Authentication",
      "summary": "Implement user authentication system",
      "done": false
    },
    {
      "key": "PROJ-101",
      "name": "API Development",
      "summary": "Build REST API",
      "done": false
    }
  ]
}
```

### Use Cases

1. **Find epic keys** to link stories to epics
2. **See active epics** in your project
3. **Identify completed epics**

### Linking to Epics

Once you find an epic key, add it to your profile:

```json
{
  "profiles": {
    "auth-story": {
      "fields": {
        "issuetype": { "id": "10002" },
        "customfield_10050": "PROJ-100"
      }
    }
  }
}
```

Note: The field name for epic link varies by Jira instance. Use `tedlt info fields` to find the correct custom field.

## Using Profiles with Info Commands

All `info` commands support the `--profile` flag:

```bash
# Use project key from the 'work' profile
tedlt info project --profile work

# Use project key from the 'personal' profile
tedlt info epics --profile personal

# Combine profile with override
tedlt info project --profile work --project-key OVERRIDE
```

This is useful when working with multiple Jira instances or projects.

## Common Workflows

### Setting Up a New Profile

1. **Discover project structure**:
   ```bash
   tedlt info project PROJ
   ```

2. **Find issue type ID** (e.g., "Bug" is `10004`)

3. **Find component IDs** (e.g., "Frontend" is `10100`)

4. **Check required fields**:
   ```bash
   tedlt info fields --project-key PROJ --issue-type 10004
   ```

5. **Create profile**:
   ```json
   {
     "profiles": {
       "bug": {
         "fields": {
           "issuetype": { "id": "10004" },
           "components": [{ "id": "10100" }]
         }
       }
     }
   }
   ```

### Copying from an Existing Ticket

1. **Find a ticket configured correctly**:
   ```bash
   tedlt info ticket PROJ-123
   ```

2. **Copy field values** to your profile

3. **Test the profile**:
   ```bash
   tedlt create "Test ticket" --profile new-profile --verbose
   ```

### Working with Custom Fields

1. **List all fields for an issue type**:
   ```bash
   tedlt info fields --project-key PROJ --issue-type 10001
   ```

2. **Find the custom field** you need (e.g., `customfield_10050` is "Epic Link")

3. **Add to profile**:
   ```json
   {
     "fields": {
       "customfield_10050": "PROJ-100"
     }
   }
   ```

## Tips and Tricks

### Save Output to a File

```bash
tedlt info project PROJ > project-info.json
```

### Use with jq for Filtering

```bash
# Extract just issue type IDs
tedlt info project PROJ | jq '.issueTypes[] | {id, name}'

# Find a specific component ID
tedlt info project PROJ | jq '.components[] | select(.name == "Frontend")'
```

### Verbose Output

Enable verbose mode to see API requests:

```bash
tedlt info project PROJ --verbose
```

## Next Steps

- **[Using Profiles](/tedlt/usage/profiles/)** - Apply the IDs you discovered to profiles
- **[Configuration Schema](/tedlt/reference/config-schema/)** - See all available field options
- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Use your configured profiles