mod evaluate;
mod token;

fn main() {
    let token_sequence = token::tokenize("0 3 +");

    match token_sequence {
        Ok(token_sequence) => {
            let result = evaluate::evaluate(token_sequence);

            match result {
                Ok(result) => println!("{result}"),
                Err(e) => println!("{e}")
            }
        }
        Err(e) => println!("{e}")
    }
}
