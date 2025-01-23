#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LABEL, ADDRESS, COMMAND, NUMBER, NEWLINE, ERROR, EOF
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub length: isize,
    pub line: isize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: isize) -> Token {
        let length = lexeme.len() as isize;

        Token {
            token_type,
            lexeme,
            length,
            line,
        }
    }
}