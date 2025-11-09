use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "tedlt")]
#[command(about = "Create Jira tickets from the command line", long_about = None)]
pub struct Args {
    #[arg(help = "The title of the Jira ticket to create")]
    pub title: String,

    #[arg(short, long)]
    pub profile: Option<String>,

    #[arg(long)]
    pub jira_url: Option<String>,

    #[arg(long)]
    pub project_key: Option<String>,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
