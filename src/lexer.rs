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
}

#[derive(Debug)]
pub enum Token {
    Operand(Operand),
    Operator(Operator),
    None
}

pub fn tokenizer(args: Vec<String>) -> Vec<Token> {
    let mut statement: Vec<Token> = Vec::new();

    for arg in args.iter() {
        let mut token = Token::None;
        for byte in arg.as_bytes() {
            match byte {
                48..=57 => {
                    match token {
                        Token::None => {
                            token = Token::Operand(
                            Operand::Numeric(Numeric {
                                value: (*byte as i64) - 48,
                            }));
                        }
                        Token::Operand(Operand::Numeric(ref mut numeric)) => {
                            numeric.value *= 10;
                            numeric.value += (*byte as i64) - 48;
                        }
                        _ => {
                            println!(
                                "{} \'{}\'",
                                "calc: Unexpected token:",
                                (*byte as char)
                            );
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    println!("calc: Invalid token: \'{}\'", arg);
                    std::process::exit(1);
                }
            }
        }
        statement.push(token);
    }
    statement
}

pub fn lexer(args: Vec<String>) -> Vec<Token> {
    tokenizer(args)
}
