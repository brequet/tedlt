---
title: Commands Reference
description: Complete reference for all tedlt commands
---

This page provides a complete reference for all tedlt commands and their options.

## Global Options

These options are available for all commands:

### `--verbose`, `-v`

Enable verbose logging to see detailed information about what tedlt is doing.

```bash
tedlt create "Fix bug" --verbose
```

### `--jira-url`

Override the Jira URL from the configuration file.

```bash
tedlt create "Task" --jira-url https://other.atlassian.net
```

This takes precedence over:
- Profile settings
- Top-level config file settings

### `--project-key`

Override the project key from the configuration file.

```bash
tedlt create "Task" --project-key OTHER
```

This takes precedence over:
- Profile settings
- Top-level config file settings

### `--help`, `-h`

Display help information for a command.

```bash
tedlt --help
tedlt create --help
tedlt info --help
```

### `--version`, `-V`

Display the version of tedlt.

```bash
tedlt --version
```

## Commands

### `create`

Create a new Jira ticket.

#### Synopsis

```bash
tedlt create <TITLE> [OPTIONS]
```

#### Arguments

**`<TITLE>`** (required)

The title/summary of the ticket to create.

```bash
tedlt create "Fix login page error"
```

Enclose the title in quotes if it contains spaces or special characters.

#### Options

**`--profile <PROFILE>`, `-p <PROFILE>`**

The name of profile(s) to use for creating the ticket. Can be specified multiple times to merge multiple profiles.

```bash
tedlt create "Fix bug" --profile bug
tedlt create "Urgent bug" --profile bug --profile urgent
```

Profiles are merged left-to-right, with later profiles overriding earlier ones.

#### Examples

```bash
# Create a simple ticket with default settings
tedlt create "Update documentation"

# Create a bug ticket
tedlt create "Login fails" --profile bug

# Create a critical bug with multiple profiles
tedlt create "Data loss issue" --profile bug --profile critical

# Create in a different project
tedlt create "Task" --project-key OTHER

# Create with verbose output
tedlt create "Debug this" --profile bug --verbose

# Create in a different Jira instance
tedlt create "Personal task" --jira-url https://personal.atlassian.net
```

#### Output

On success, displays the created ticket key:

```
Created ticket: PROJ-123
```

With `--verbose`, displays additional information:
- Configuration resolution
- Profile merging
- API request/response
- Full ticket details

---

### `info`

Discover Jira metadata for projects, epics, fields, and more.

The `info` command has several subcommands for different types of information.

#### Alias

`discover` is an alias for `info`:

```bash
tedlt discover project PROJ
```

#### Global Info Options

**`--profile <PROFILE>`**

Use profile(s) for context (e.g., to get the project key).

```bash
tedlt info project --profile work
```

---

### `info project`

Fetch metadata for a specific project.

#### Synopsis

```bash
tedlt info project [KEY] [OPTIONS]
```

#### Arguments

**`[KEY]`** (optional)

The project key (e.g., "PROJ", "KAN").

If not provided, uses the project key from:
1. `--profile` option
2. `--project-key` option
3. Configuration file

```bash
tedlt info project PROJ
tedlt info project --project-key PROJ
tedlt info project --profile work
```

#### Output

Displays project metadata including:
- Project name and key
- Available issue types and their IDs
- Components and their IDs
- Versions and their IDs
- Other project settings

#### Example Output

```json
{
  "key": "PROJ",
  "name": "My Project",
  "issueTypes": [
    { "id": "10001", "name": "Task" },
    { "id": "10004", "name": "Bug" },
    { "id": "10002", "name": "Story" }
  ],
  "components": [
    { "id": "10100", "name": "Frontend" },
    { "id": "10101", "name": "Backend" }
  ],
  "versions": [
    { "id": "10200", "name": "v1.0.0" }
  ]
}
```

#### Examples

```bash
# Get project info by key
tedlt info project PROJ

# Get project info using profile
tedlt info project --profile work

# Get project info with verbose output
tedlt info project PROJ --verbose
```

---

### `info fields`

Fetch metadata about fields for a given issue type in a project.

#### Synopsis

```bash
tedlt info fields [OPTIONS]
```

#### Options

**`--project-key <KEY>`**

The project key (e.g., "PROJ").

**`--issue-type <ID>`**

The ID of the issue type (e.g., "10001").

Both options are required to get meaningful results.

#### Output

Displays field metadata including:
- Field key and name
- Whether the field is required
- Field type
- Allowed values (for select fields)

#### Example Output

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
      "key": "priority",
      "name": "Priority",
      "required": false,
      "schema": { "type": "priority" },
      "allowedValues": [
        { "id": "1", "name": "Highest" },
        { "id": "2", "name": "High" },
        { "id": "3", "name": "Medium" }
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

#### Examples

```bash
# Get fields for a specific issue type
tedlt info fields --project-key PROJ --issue-type 10001

# Get fields using profile for context
tedlt info fields --issue-type 10001 --profile work
```

---

### `info ticket`

Inspect the raw JSON data of an existing ticket.

#### Synopsis

```bash
tedlt info ticket <KEY> [OPTIONS]
```

#### Arguments

**`<KEY>`** (required)

The ticket key (e.g., "PROJ-123").

```bash
tedlt info ticket PROJ-123
```

#### Output

Displays the complete JSON representation of the ticket, including:
- All field values
- Custom field data
- Issue type, priority, status
- Components, labels, versions
- Assignee, reporter
- Comments metadata
- Links to other issues

#### Example Output

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
    "status": {
      "name": "In Progress"
    },
    "labels": ["bug", "frontend"],
    "components": [
      { "id": "10100", "name": "Frontend" }
    ],
    "customfield_10050": "PROJ-100"
  }
}
```

#### Use Cases

1. **Reverse-engineer configurations**: Copy field values into your profiles
2. **Find custom field IDs**: See which `customfield_*` corresponds to which field
3. **Debug ticket creation**: Compare with your configuration

#### Examples

```bash
# View ticket details
tedlt info ticket PROJ-123

# View with verbose output
tedlt info ticket PROJ-123 --verbose
```

---

### `info boards`

List all available boards.

#### Synopsis

```bash
tedlt info boards [OPTIONS]
```

#### Options

**`--project <KEY>`**

Optionally filter boards by project key.

```bash
tedlt info boards --project PROJ
```

#### Output

Displays a list of boards with their IDs, names, and types.

#### Example Output

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

#### Examples

```bash
# List all boards
tedlt info boards

# List boards for a specific project
tedlt info boards --project PROJ

# List boards with verbose output
tedlt info boards --verbose
```

---

### `info epics`

List epics for a given board or project.

#### Synopsis

```bash
tedlt info epics [OPTIONS]
```

#### Options

**`--project-key <KEY>`**

The project key to find epics for.

**`--board-id <ID>`**

The board ID to find epics for.

You can specify either option, but not both.

#### Output

Displays a list of epics with their keys, names, and status.

#### Example Output

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

#### Examples

```bash
# List epics for a project
tedlt info epics --project-key PROJ

# List epics for a board
tedlt info epics --board-id 1

# List epics using profile for context
tedlt info epics --profile work

# List epics with verbose output
tedlt info epics --project-key PROJ --verbose
```

---

### `init`

Initialize the configuration file in the home directory.

#### Synopsis

```bash
tedlt init [OPTIONS]
```

#### Options

**`--jira-url <URL>`, `-j <URL>`**

The Jira URL to use in the configuration file.

```bash
tedlt init --jira-url https://company.atlassian.net
```

**`--project-key <KEY>`, `-p <KEY>`**

The project key to use in the configuration file.

```bash
tedlt init --project-key PROJ
```

**`--force`, `-f`**

Force overwrite of an existing configuration file.

```bash
tedlt init --force
```

Without this flag, `init` will refuse to overwrite an existing config file.

#### Behavior

1. Creates the `.tedlt` directory in your home directory (if it doesn't exist)
2. Creates a `config.json` file with the provided values
3. If `--jira-url` and `--project-key` are not provided, prompts for them interactively

#### Output

```
Configuration file created at: /home/user/tedlt.jsonc
```

#### Examples

```bash
# Initialize with prompts
tedlt init

# Initialize with values
tedlt init --jira-url https://company.atlassian.net --project-key PROJ

# Force overwrite existing config
tedlt init --force

# Initialize with just the URL (will prompt for project key)
tedlt init --jira-url https://company.atlassian.net
```

---

## Environment Variables

While not commands, these environment variables control tedlt's behavior:

### `JIRA_API_TOKEN`

Your Jira API token for authentication.

See [Environment Variables](/tedlt/configuration/environment/) for more details.

---

## Next Steps

- **[Creating Tickets](/tedlt/usage/creating-tickets/)** - Learn how to use the create command
- **[Info Commands](/tedlt/usage/info-commands/)** - Learn how to use the info commands
- **[Configuration](/tedlt/configuration/overview/)** - Configure tedlt for your workflow
