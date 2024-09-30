use crate::token::{Operator, Token};

fn pop_operand(stack: &mut Vec<f64>) -> Result<f64, String> {
    match stack.pop() {
        Some(operand) => Ok(operand),
        None => Err(String::from(
            "Error evaluating token sequence: stack is empty.",
        )),
    }
}

pub fn evaluate(token_sequence: &Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in token_sequence {
        match token {
            Token::Operand(number) => stack.push(*number),

            Token::Operator(operator) => {
                let right_operand = pop_operand(&mut stack)?;
                let left_operand = pop_operand(&mut stack)?;

                match operator {
                    Operator::Add => stack.push(left_operand + right_operand),
                    Operator::Subtract => stack.push(left_operand - right_operand),
                    Operator::Multiply => stack.push(left_operand * right_operand),
                    Operator::Divide => {
                        if right_operand == 0.0 {
                            return Err(String::from(
                                "Error evaluating token sequence: division by zero.",
                            ));
                        }

                        stack.push(left_operand / right_operand);
                    }
                }
            }
        }
    }

    pop_operand(&mut stack)
}
