mod lexer;
mod evaluator;
mod token;

use clap::Parser;

#[derive(Parser)]
#[command(name = "calc", arg_required_else_help = true, version, about, long_about = None)]
struct Cli {
    equation: Vec<String>
}


// ---------------------------
// This code was provided by ChatGPT, an AI language model developed by OpenAI.
#[macro_export]
macro_rules! err {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        {
            eprint!("calc: ");
            eprintln!($fmt $(, $($arg)*)?);
            std::process::exit(1);
        }
    };
}
// ---------------------------

fn main() {
    let cli = Cli::parse();
    println!("{}", evaluator::evaluate(lexer::lexer(cli.equation)));
}
