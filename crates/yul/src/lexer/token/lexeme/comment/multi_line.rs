//! The multi-line comment lexeme.

use crate::lexer::token::lexeme::Lexeme;
use crate::lexer::token::location::Location;
use crate::lexer::token::Token;

/// The multi-line comment lexeme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {}

impl Comment {
    /// The start symbol.
    pub const START: &'static str = "/*";
    /// The end symbol.
    pub const END: &'static str = "*/";

    /// Returns the comment, including its length and number of lines.
    pub fn parse(input: &str) -> Token {
        let end_position = input.find(Self::END).unwrap_or(input.len());
        let input = &input[..end_position];

        let length = (end_position + Self::END.len())
            .try_into()
            .expect("the YUL should be of reasonable size");
        let lines = input
            .matches('\n')
            .count()
            .try_into()
            .expect("the YUL should be of reasonable size");
        let columns = match input.rfind('\n') {
            Some(new_line) => end_position - (new_line + 1),
            None => end_position,
        }
        .try_into()
        .expect("the YUL should be of reasonable size");

        Token::new(Location::new(lines, columns), Lexeme::Comment, length)
    }
}
