use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Add { name } => {
            println!("'clappy-clap-learning' was used, name is {name:?}");
        }
    }
}
