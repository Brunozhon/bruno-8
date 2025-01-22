# Instructions

| Opcode | Instruction | Arguments        |
|--------|-------------|------------------|
| `0x00` | HLT         |
| `0x01` | ADD         |
| `0x02` | SUB         |
| `0x03` | MUL         |
| `0x04` | DIV         |
| `0x05` | PUSH        | 16-bit address   |
| `0x06` | POP         | 16-bit address   |
| `0x07` | AND         |
| `0x08` | NAND        |
| `0x09` | OR          |
| `0x0A` | XOR         |
| `0x0B` | NOR         |
| `0x0C` | NOT         |
| `0x0D` | INC         |
| `0x0E` | DEC         |
| `0x0F` | NOP         |
| `0x10` | PSP         |
| `0x11` | SSP         |
| `0x12` | EQ          |
| `0x13` | GT          |
| `0x14` | LT          |
| `0x15` | GEQ         |
| `0x16` | LEQ         |
| `0x17` | NEQ         |
| `0x18` | JMP         | 16-bit address   |
| `0x19` | JNZ         | 16-bit address   |
| `0x1A` | JEZ         | 16-bit address   |
| `0x1B` | IMS         | 8-bit immediate* |
| `0x1C` | PTV         |

*Immediates are unsigned integers.