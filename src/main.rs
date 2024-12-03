use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(commands) = &cli.commands {
        match commands {
            Commands::Add(name) => {
                println!("'clappy-clap-learning' was used, name is {:?}", name.name);
            }
        }
    }
}
