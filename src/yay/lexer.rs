use crate::yay::token::{Token, TokenType};

pub struct Lexer {
    start: usize,
    current: usize,
    source: Vec<char>,
    line: isize
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            start: 0,
            current: 0,
            source: source.chars().collect(),
            line: 1
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current += 1;
            self.source[self.current - 1]
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        let c = self.advance();

        match c {
            '.' => self.label(),
            '$' => self.memory_address(),
            _ => self.error_token("Unexpected character")
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.source[self.start..self.current].iter().collect::<String>(), self.line)
    }

    fn error_token(&self, message: &str) -> Token {
        Token::new(TokenType::ERROR, message.to_string(), self.line)
    }

    fn memory_address(&mut self) -> Token {
        while self.peek() >= '0' && self.peek() <= '9' && !self.is_at_end() {
            self.advance();
        }

        self.make_token(TokenType::ADDRESS)
    }

    fn label(&mut self) -> Token {
        while self.peek() >= 'a' && self.peek() <= 'z' && !self.is_at_end() {
            self.advance();
        }

        self.make_token(TokenType::LABEL)
    }
}