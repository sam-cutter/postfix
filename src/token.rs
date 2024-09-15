pub enum Token {
    Operand(f64),
    Operator(Operator),
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut token_sequence: Vec<Token> = Vec::new();

    let words = expression.trim().split(' ');

    for word in words {
        let token: Result<Token, String> = match word {
            "+" => Ok(Token::Operator(Operator::Add)),
            "-" => Ok(Token::Operator(Operator::Subtract)),
            "*" => Ok(Token::Operator(Operator::Multiply)),
            "/" => Ok(Token::Operator(Operator::Divide)),

            _ => match word.parse::<f64>() {
                Ok(operand) => Ok(Token::Operand(operand)),
                Err(_) => Err(format!(
                    "Error tokenizing expression: \"{word}\" is not a valid token."
                )),
            },
        };

        token_sequence.push(token?);
    }

    return Ok(token_sequence);
}
