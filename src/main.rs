use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    /// Network port to use
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("PORT = {}", cli.port);
}
