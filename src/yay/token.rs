#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    LABEL,
    ADDRESS,
    NUMBER,

    // Instructions
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

    // Miscellaneous
    NEWLINE,
    ERROR,
    EOF
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