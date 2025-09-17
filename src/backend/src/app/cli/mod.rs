use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Migrate the user
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
#[command(rename_all = "kebab-case")]
pub enum Command {
    /// Sort out the users
    SeedLessons(SeedArgs),
}

#[derive(Parser, Debug)]
pub struct SeedArgs {
    /// Set to true if you intend to actually writing to the database
    #[clap(short, long)]
    pub write: bool,
}
