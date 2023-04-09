#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Clone(CloneArgs),
    Look(LookArgs),
    List(ListArgs),
    /// Print clg root directory
    Root,
}

#[derive(Debug, clap::Args)]
pub struct CloneArgs {
    /// Name of local repository
    #[arg(short = 'n', long)]
    pub name: Option<String>,
    pub url: String,
}

#[derive(Debug, clap::Args)]
pub struct LookArgs {
    pub repository: String,
}

#[derive(Debug, clap::Args)]
pub struct ListArgs {
    /// Generate repository list for completion
    #[arg(long)]
    pub completion: bool,
}
