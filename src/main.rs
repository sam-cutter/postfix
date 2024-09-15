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

fn evaluate(expression: Vec<Token>) {
}
    
    
fn main() {
    let tokens = tokenize(String::from("1 2 +"));

    match tokens {
        Ok(tokens) => evaluate(tokens),
        Err(e) => println!("{e}"),
    }
}
