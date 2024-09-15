mod token;

use token::{Operator, Token};

fn evaluate(expression: Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression {
        match token {
            Token::Operand(number) => stack.push(number),
            Token::Operator(operator) => {
                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap();

                match operator {
                    Operator::Add => stack.push(first + second),
                    Operator::Subtract => stack.push(first - second),
                    Operator::Multiply => stack.push(first * second),
                    Operator::Divide => stack.push(first / second),
                }
            }
        }
    }

    return stack.pop().unwrap();
}

fn main() {
    let tokens = token::tokenize("9 3 /");

    match tokens {
        Ok(tokens) => println!("{}", evaluate(tokens)),
        Err(e) => println!("{e}"),
    }
}
