use crate::parser::Parser;

mod ast;
mod token;
mod lexer;
mod parser;

fn main() {
    let inputs = [
        "2 + 3",
        "2 + 3 * 4",
        "10 - 2 - 3",
        "(2 + 3) * 4",
        "-5",
        "--5",
        "2 * -3",
    ];
    for input in inputs {
        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(expr) => println!("expr: {:?} = {:?}", expr.to_string(), expr.eval()),
            Err(e)  => println!("error: {:?}", e.message)
        }
    }
}
