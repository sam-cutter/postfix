use crate::lexer::{Operator, Token};

pub enum EvaluatorError {
    EmptyStack,
    DivisionByZero,
}

fn pop_operand(stack: &mut Vec<f64>) -> Result<f64, EvaluatorError> {
    match stack.pop() {
        Some(operand) => Ok(operand),
        None => Err(EvaluatorError::EmptyStack),
    }
}

pub fn evaluate(token_sequence: &Vec<Token>) -> Result<f64, EvaluatorError> {
    let mut stack: Vec<f64> = Vec::new();

    for token in token_sequence {
        match token {
            Token::Operand(number) => stack.push(*number),

            Token::Operator(operator) => {
                let right_operand = pop_operand(&mut stack)?;
                let left_operand = pop_operand(&mut stack)?;

                match operator {
                    Operator::Addition => stack.push(left_operand + right_operand),
                    Operator::Subtraction => stack.push(left_operand - right_operand),
                    Operator::Multiplication => stack.push(left_operand * right_operand),
                    Operator::Division => {
                        if right_operand == 0.0 {
                            return Err(EvaluatorError::DivisionByZero);
                        }

                        stack.push(left_operand / right_operand);
                    }
                }
            }
        }
    }

    pop_operand(&mut stack)
}
