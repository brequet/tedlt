# tedlt - Tout Est Dans Le Titre

A CLI tool to create Jira tickets from the command line with just a title.

**"Everything is in the title"** - Create Jira tickets fast, without opening a browser.

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

### Setup

1. Create a `.env` file with your Jira credentials:
   ```bash
   JIRA_API_TOKEN=your_api_token_here
   JIRA_EMAIL=your.email@example.com
   ```

2. Initialize configuration:
   ```bash
   tedlt init
   ```

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

- [Installation Guide](https://brequet.github.io/tedlt/getting-started/installation/)
- [Quick Start](https://brequet.github.io/tedlt/getting-started/quick-start/)
- [Configuration](https://brequet.github.io/tedlt/configuration/overview/)
- [CLI Reference](https://brequet.github.io/tedlt/reference/commands/)

## Example Configuration

```jsonc
{
  "jira_url": "https://yourcompany.atlassian.net",
  "project_key": "PROJ",
  
  "properties": {
    "team_lead": "USER123",
    "default_priority": "3"
  },
  
  "profiles": {
    "default": {
      "fields": {
        "priority": { "id": "{{default_priority}}" },
        "labels": ["auto-created"]
      }
    },
    "bug": {
      "fields": {
        "issuetype": { "id": "10004" },
        "labels": ["bug"]
      }
    },
    "bug-critical": {
      "inherits": ["bug"],
      "fields": {
        "priority": { "id": "1" },
        "labels": ["urgent"]
      }
    }
  }
}
```

## License

This project is open source. See the repository for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.