use crate::emulator::Emulator;
use crate::emulator::instruction::Instruction;

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