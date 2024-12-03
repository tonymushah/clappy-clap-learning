use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    ///
    /// Ignored paragraph
    Slow,
}

fn main() {
    let cli = Cli::parse();
    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}
