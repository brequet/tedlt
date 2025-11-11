use clap::{Parser, Subcommand};

/// A CLI tool to interact with Jira and create tickets efficiently.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    after_help = "EXAMPLES:\n  \
        tedlt create \"Fix login bug\" --profile work\n  \
        tedlt info project KAN\n  \
        tedlt info ticket KAN-123"
)]
pub struct Args {
    /// Enable verbose logging.
    #[arg(short, long, global = true, default_value_t = false)]
    pub verbose: bool,

    /// Override the Jira URL from the config file.
    #[arg(long, global = true)]
    pub jira_url: Option<String>,

    /// Override the project key from the config file or profile.
    #[arg(long, global = true)]
    pub project_key: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Jira ticket.
    Create(CreateCommand),

    /// Discover Jira metadata for projects, epics, and more.
    #[command(alias = "discover")]
    Info(InfoCommand),
}

/// Arguments for the 'create' command.
#[derive(Parser, Debug)]
pub struct CreateCommand {
    /// The title of the ticket to create.
    #[arg(required = true)]
    pub title: String,

    /// The name of profile(s) to use for creating the ticket.
    /// Can be specified multiple times. Profiles are merged left-to-right.
    #[arg(short, long, value_name = "PROFILE")]
    pub profile: Vec<String>,
}

/// Arguments for the 'info' command.
#[derive(Parser, Debug)]
pub struct InfoCommand {
    #[command(subcommand)]
    pub subcmd: InfoSubCommand,

    /// The name of profile(s) to use for context (e.g., project key).
    /// Can be specified multiple times. Profiles are merged left-to-right.
    #[arg(long, global = true, value_name = "PROFILE")]
    pub profile: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum InfoSubCommand {
    /// Fetch metadata for a specific project.
    ///
    /// Displays available issue types, components, and versions.
    Project {
        /// The project key (e.g., "KAN").
        key: Option<String>,
    },

    /// List epics for a given board or project.
    ///
    /// Helps find the correct epic ID to link new stories to.
    Epics {
        /// The project key to find epics for.
        #[arg(long)]
        project_key: Option<String>,
        /// The board ID to find epics for.
        #[arg(long)]
        board_id: Option<u64>,
    },

    /// Inspect the raw JSON data of an existing ticket.
    ///
    /// Useful for reverse-engineering field names and values.
    Ticket {
        /// The ticket key (e.g., "KAN-123").
        #[arg(required = true)]
        key: String,
    },

    /// List all available boards.
    ///
    /// Useful for finding board IDs required by certain Jira APIs.
    Boards {
        /// Optionally filter boards by a project key.
        #[arg(long)]
        project: Option<String>,
    },
    /// Fetch metadata about the fields for a given issue type in a project.
    ///
    /// Displays available fields, whether they are required, and their types.
    Fields {
        /// The project key (e.g., "KAN").
        #[arg(long)]
        project_key: Option<String>,
        /// The ID of the issue type.
        #[arg(long)]
        issue_type: Option<String>,
    },
}

impl Args {
    pub fn parse_args() -> Self {
        Parser::parse()
    }
}
