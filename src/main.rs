use inquire::Text;
use postfix::{evaluate, token};

fn main() {
    loop {
        let expression =
            match Text::new("Enter expression to be evaluated (enter to exit): ").prompt() {
                Ok(expression) => {
                    if expression == "" {
                        break;
                    }

                    expression
                }
                Err(_) => {
                    println!("Error!");
                    continue;
                }
            };

        let token_sequence = match token::tokenize(&expression) {
            Ok(token_sequence) => token_sequence,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        let evaluation = match evaluate::evaluate(&token_sequence) {
            Ok(evaluation) => evaluation,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        println!("{} = {}", expression, evaluation);
    }
}
