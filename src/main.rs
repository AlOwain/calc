use clap::{Parser};

#[derive(Parser)]
#[command(name = "calc")]
#[command(version, about, long_about = None)]
struct Cli {
    equation: Vec<String>
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli.equation);
}
