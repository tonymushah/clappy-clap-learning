use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[arg(default_value_t = 2024)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("port: {:?}", cli.port);
}
