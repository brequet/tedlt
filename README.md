# tedlt

A simple CLI tool to create Jira tickets from the command line with just a title.

## Features

- Create Jira tickets with a single command
- Automatic configuration discovery (walks up directory tree)
- Secure API token management via environment variables
- Clean error handling and logging
- Idiomatic Rust implementation

## Installation

### From Source

```bash
git clone <repository-url>
cd tedlt
cargo build --release
```

The binary will be available at `target/release/tedlt`.

Optionally, install it globally:

```bash
cargo install --path .
```

## Configuration

### 1. Run the Tool Once to Generate Config

On first run, `tedlt` will automatically create a config file at `~/tedlt.toml`:

```bash
tedlt "My first ticket"
```

This will create the default config and prompt you to edit it.

### 2. Edit Your Configuration File

Edit `~/tedlt.toml` with your Jira settings:

```toml
jira_url = "https://requet.atlassian.net"
project_key = "KAN"
```

### 3. Set Up Environment Variables

Create a `.env` file in your project root:

```env
JIRA_API_TOKEN=your_api_token_here
JIRA_EMAIL=your.email@example.com
```

You can use the provided example:

```bash
cp .env.example .env
```

#### Getting Your Jira API Token

1. Go to https://id.atlassian.com/manage-profile/security/api-tokens
2. Click "Create API token"
3. Give it a name (e.g., "tedlt")
4. Copy the token and paste it into your `.env` file

**Important:** Keep your `.env` file secure and never commit it to version control!

## Usage

Create a Jira ticket with a title:

```bash
tedlt "Implement user authentication"
```

The tool will:
1. Load configuration from `~/tedlt.toml` (creates it on first run if missing)
2. Load credentials from `.env` or environment variables
3. Create a Story ticket (type 10004) in your configured Jira project with label "bobby"
4. Print the ticket URL (e.g., `https://requet.atlassian.net/browse/KAN-123`)

### Logging

Control log verbosity with the `RUST_LOG` environment variable:

```bash
# Show only errors and info (default)
tedlt "My ticket title"

# Show debug information
RUST_LOG=debug tedlt "My ticket title"

# Disable all logging
RUST_LOG=error tedlt "My ticket title"
```

## Project Structure

```
tedlt/
├── src/
│   ├── main.rs       # Application entry point and orchestration
│   ├── cli.rs        # Command-line argument parsing
│   ├── config.rs     # TOML configuration loading
│   ├── env.rs        # Environment variable handling
│   └── jira.rs       # Jira API client
├── .env.example      # Environment variable template
├── tedlt.toml.example # Bundled default configuration (auto-created at ~/tedlt.toml)
└── Cargo.toml        # Dependencies and project metadata
```

## How It Works

1. **First Run**: The app bundles `tedlt.toml.example` and creates `~/tedlt.toml` if it doesn't exist
2. **Configuration Loading**: Always reads from `~/tedlt.toml` (one central config location)
3. **No Setup Required**: Just run the tool, edit the generated config, and you're ready to go

## Error Handling

The tool provides clear error messages for common issues:

- Missing or invalid configuration file
- Missing environment variables
- Jira API errors
- Network connectivity issues

All errors are properly propagated and logged with context.

## Requirements

- Rust 1.70 or later
- Active Jira Cloud account
- Valid API token

## License

[Your License Here]