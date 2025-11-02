use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tedlt")]
#[command(about = "Create Jira tickets from the command line", long_about = None)]
pub struct Args {
    #[arg(help = "The title/summary of the Jira ticket to create")]
    pub title: String,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
