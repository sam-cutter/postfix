use inquire::Text;
use postfix::{evaluator, lexer};

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

        let lexical_token_sequence = match lexer::lex(&expression) {
            Ok(lexical_token_sequence) => lexical_token_sequence,
            Err(_) => {
                println!("Error lexing expression.");
                continue;
            }
        };

        let evaluation = match evaluator::evaluate(&lexical_token_sequence) {
            Ok(evaluation) => evaluation,
            Err(_) => {
                println!("Error evaluating lexical token sequence.");
                continue;
            }
        };

        println!("{} = {}", expression, evaluation);
    }
}
