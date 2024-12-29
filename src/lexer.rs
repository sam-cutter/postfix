const ADDITION: &str = "+";
const SUBTRACTION: &str = "-";
const MULTIPLICATION: &str = "*";
const DIVISION: &str = "/";

pub enum LexerError {
    InvalidToken,
}

pub enum Token {
    Operand(f64),
    Operator(Operator),
}

pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub fn lex(expression: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexical_tokens: Vec<Token> = Vec::new();

    let tokens = expression.trim().split_whitespace();

    for token in tokens {
        let lexical_token = match token {
            ADDITION => Ok(Token::Operator(Operator::Addition)),
            SUBTRACTION => Ok(Token::Operator(Operator::Subtraction)),
            MULTIPLICATION => Ok(Token::Operator(Operator::Multiplication)),
            DIVISION => Ok(Token::Operator(Operator::Division)),

            _ => match token.parse::<f64>() {
                Ok(operand) => Ok(Token::Operand(operand)),
                Err(_) => Err(LexerError::InvalidToken),
            },
        };

        lexical_tokens.push(lexical_token?);
    }

    Ok(lexical_tokens)
}
