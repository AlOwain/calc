#[cfg(test)]

// TODO:    Create some random generator for equations and check for:
//          - Proper evaluation of the final result
//          - In `to_postfix()`, you can be sure that the order of the operands would be exactly the same infixed as it is postfixed.
//          - Create a test that generates large digits and tests its parsing
//          ...

mod evaluator;
mod lexer;
mod token;
