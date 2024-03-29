mod lexer;
mod evaluator;

use clap::Parser;

#[derive(Parser)]
#[command(name = "calc", arg_required_else_help = true, version, about, long_about = None)]
struct Cli {
    equation: Vec<String>
}

#[derive(Debug)]
pub enum Operator {
    Add,
}

#[derive(Debug)]
pub struct Numeric {
    value: i64,
}

#[derive(Debug)]
pub enum Operand {
    Numeric(Numeric),
    None
}

impl Operand {
    fn new(value: String) -> Self {
        // TODO Convert 
        let value = match value.parse() {
            Ok(val) => val,
            Err(err) => {
                panic!("calc: {} '{}'\n{}",
                    "Unexpected value given as operand", value, err
                );
            }
        };
        Operand::Numeric(Numeric {
            value,
        })
    }
}

#[derive(Debug)]
pub enum Token {
    Operand(Operand),
    Operator(Operator),
    None
}


fn main() {
    let cli = Cli::parse();
    dbg!(evaluator::evaluate(lexer::lexer(cli.equation)));
}
