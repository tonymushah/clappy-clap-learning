use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    println!(
        "verbose {} time{}...",
        cli.verbose,
        if cli.verbose < 2 { "" } else { "s" }
    );

    println!("name: {:?}", cli.name);
}
