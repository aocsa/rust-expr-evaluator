use crate::parser::Parser;

mod ast;
mod error;
mod lexer;
mod parser;
mod token;

fn main() {
    let inputs = [
        "2 + 3",
        "2 + 3 * 4",
        "10 - 2 - 3",
        "(2 + 3) * 4",
        "-5",
        "--5",
        "2 * -3",
        "10 / 0", // Division by zero test
        "1 + @",  // Lexer error test
        "(2 + 3", // Unbalanced parenthesis test
    ];

    for input in inputs {
        print!("Input: {:?} => ", input);
        match Parser::new(input) {
            Ok(mut parser) => match parser.parse() {
                Ok(expr) => match expr.eval() {
                    Ok(value) => println!("{} = {}", expr.to_string(), value),
                    Err(e) => println!("Evaluation error: {}", e),
                },
                Err(e) => println!("Parse error: {}", e),
            },
            Err(e) => println!("Lexer error: {}", e),
        }
    }
}
