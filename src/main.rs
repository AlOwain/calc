mod lexer;

use clap::Parser;

#[derive(Parser)]
#[command(name = "calc", arg_required_else_help = true, version, about, long_about = None)]
struct Cli {
    equation: Vec<String>
}

fn main() {
    let cli = Cli::parse();
    dbg!(lexer::lexer(cli.equation));
}
