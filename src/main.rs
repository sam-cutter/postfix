enum Token {
    Operand(f64),
    Operator(Operator)
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn tokenize(input: String) -> Result<Vec<Token>, String> {

    let mut tokens: Vec<Token> = Vec::new();

    for word in input.split(' ') {
        let token: Result<Token, String> = match word {
            "+" => Ok(Token::Operator(Operator::Add)),
            "-" => Ok(Token::Operator(Operator::Subtract)),
            "*" => Ok(Token::Operator(Operator::Multiply)),
            "/" => Ok(Token::Operator(Operator::Divide)),
            _ =>  match word.parse::<f64>() {
                Ok(number) => Ok(Token::Operand(number)),
                Err(_) => Err(format!("Error parsing token: {word} is not a valid token."))
            }
        };

        match token {
            Ok(token) => tokens.push(token),
            Err(e) => return Err(e)
        }

    }

    return Ok(tokens);
}

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
    let tokens = tokenize(String::from("9 3 /"));

    match tokens {
        Ok(tokens) => println!("{}", evaluate(tokens)),
        Err(e) => println!("{e}"),
    }
}
