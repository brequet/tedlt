# tedlt - Tout Est Dans Le Titre

A CLI tool to create Jira tickets from the command line with just a title.

**"Everything is in the title"** - Create Jira tickets fast, without opening a browser.

See complete documentation at: https://brequet.github.io/tedlt/.

## Features

- 🚀 **Fast ticket creation** - Create tickets with a single command
- 🔧 **Powerful configuration** - Use profiles to customize ticket fields
- 🔄 **Profile inheritance** - Build complex configurations from simple building blocks
- 📝 **Property templates** - Reuse values with variable substitution
- 🌐 **Multi-instance support** - Work with different Jira instances
- 📊 **Information retrieval** - Query projects, epics, boards, and fields

## Quick Start

### Installation

Download the latest release for your platform from the [releases page](https://github.com/brequet/tedlt/releases/latest).

### Usage

Create a ticket:
```bash
tedlt create "Fix login page error"
```

Create with a profile:
```bash
tedlt create "Fix bug" --profile bug
```

View project information:
```bash
tedlt info project PROJ
```

## Documentation

📚 **[Full Documentation](https://brequet.github.io/tedlt/)**

## Example Configuration

```jsonc
{
  "jira_url": "https://acmecorp.atlassian.net",
  "project_key": "SHOP",
  "properties": {
    "defaultEpic": "SHOP-42",        // Shopping Cart Epic
    "defaultVersion": "10523",       // Q1 2024 Release
    "issueTypes": {
      "story": "10001",
      "bug": "10004",
      "task": "10002"
    }
  },
  "profiles": {
    "default": { // Default profile applied to all tickets
      "fields": {
        "parent": { "id": "${defaultEpic}" },
        "fixVersions": [{ "id": "${defaultVersion}" }],
        "issuetype": { "id": "${issueTypes.story}" }
      }
    },
    "backend": {
      "fields": {
        "components": [{ "id": "11001" }]  // Backend Component
      }
    },
    "frontend": {
      "fields": {
        "components": [{ "id": "11002" }]  // Frontend Component
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "${issueTypes.bug}" },
        "priority": { "id": "3" }  // High priority
      }
    }
  }
}
```

## License

This project is open source. See the repository for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
