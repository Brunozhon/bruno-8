use crate::emulator::Emulator;
use crate::emulator::instruction::Instruction;
use crate::yay::lexer::Lexer;
use crate::yay::token::TokenType;

#[test]
fn addition() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(3);
    code.push(Instruction::IMS.into());
    code.push(2);
    code.push(Instruction::ADD.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 5);
}

#[test]
fn subtraction() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(5);
    code.push(Instruction::IMS.into());
    code.push(3);
    code.push(Instruction::SUB.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 2);
}

#[test]
fn multiplication() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(2);
    code.push(Instruction::IMS.into());
    code.push(5);
    code.push(Instruction::MUL.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 10);
}

#[test]
fn division() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(8);
    code.push(Instruction::IMS.into());
    code.push(2);
    code.push(Instruction::DIV.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 4);
}

#[test]
fn xor() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(10);
    code.push(Instruction::IMS.into());
    code.push(10);
    code.push(Instruction::XOR.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 0);
}

#[test]
fn and() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(3);
    code.push(Instruction::IMS.into());
    code.push(2);
    code.push(Instruction::AND.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 2);
}

#[test]
fn or() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(3);
    code.push(Instruction::IMS.into());
    code.push(2);
    code.push(Instruction::OR.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 3);
}

#[test]
fn push_pop() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(123);
    code.push(Instruction::POP.into());
    code.push(128);
    code.push(0);
    code.push(Instruction::PUSH.into());
    code.push(128);
    code.push(0);

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 123);
}

#[test]
fn push_to_void() {
    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(123);
    code.push(Instruction::IMS.into());
    code.push(255);
    code.push(Instruction::PTV.into());

    let mut emulator = Emulator::new(code);
    emulator.run_unwindowed();

    assert_eq!(emulator.pop_stack(), 123);
}

#[test]
fn lex_address() {
    let mut lexer = Lexer::new(String::from("$1234"));
    let token = lexer.scan_token();

    assert_eq!(token.lexeme, String::from("$1234"));
    assert_eq!(token.token_type, TokenType::ADDRESS);
    assert_eq!(token.length, 5);
    assert_eq!(token.line, 1);
}

#[test]
fn lex_label() {
    let mut lexer = Lexer::new(String::from(".label"));
    let token = lexer.scan_token();

    assert_eq!(token.lexeme, String::from(".label"));
    assert_eq!(token.token_type, TokenType::LABEL);
    assert_eq!(token.length, 6);
    assert_eq!(token.line, 1);
}

#[test]
fn lex_number() {
    let mut lexer = Lexer::new(String::from("123"));
    let token = lexer.scan_token();

    assert_eq!(token.lexeme, String::from("123"));
    assert_eq!(token.token_type, TokenType::NUMBER);
    assert_eq!(token.length, 3);
    assert_eq!(token.line, 1);
}

#[test]
fn lex_instruction() {
    let mut lexer = Lexer::new(String::from("push"));
    let token = lexer.scan_token();

    assert_eq!(token.lexeme, String::from("push"));
    assert_eq!(token.token_type, TokenType::PUSH);
    assert_eq!(token.length, 4);
    assert_eq!(token.line, 1);
}

#[test]
fn skip_whitespace() {
    let mut lexer = Lexer::new(String::from("\n"));
    let token = lexer.scan_token();

    assert_eq!(token.lexeme, String::from(""));
    assert_eq!(token.token_type, TokenType::EOF);
    assert_eq!(token.length, 0);
    assert_eq!(token.line, 2);
}