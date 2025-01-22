use std::mem::transmute;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Instruction {
    HLT,
    ADD,
    SUB,
    MUL,
    DIV,
    PUSH,
    POP,
    AND,
    NAND,
    OR,
    XOR,
    NOR,
    NOT,
    INC,
    DEC,
    NOP,
    PSP,
    SSP,
    FLG,
    CFLG,
    SFLG,
    EQ,
    GT,
    LT,
    GEQ,
    LEQ,
    NEQ,
    JMP,
    JNZ,
    JEZ,
    IMS,
    PTV,
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Instruction {
    fn from(byte: u8) -> Self {
        if byte > 31 {
            panic!("Invalid instruction byte: {}", byte);
        }
        unsafe { transmute(byte) }
    }
}