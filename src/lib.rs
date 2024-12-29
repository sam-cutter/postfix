pub mod evaluator;
pub mod lexer;

pub enum PostfixError {
    LexerError,
    EvaluatorError,
}
