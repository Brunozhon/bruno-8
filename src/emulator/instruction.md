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
| `0x12` | FLG         |
| `0x13` | CFLG        |
| `0x14` | SFLG        |
| `0x15` | EQ          |
| `0x16` | GT          |
| `0x17` | LT          |
| `0x18` | GEQ         |
| `0x19` | LEQ         |
| `0x1A` | NEQ         |
| `0x1B` | JMP         | 16-bit address   |
| `0x1C` | JNZ         | 16-bit address   |
| `0x1D` | JEZ         | 16-bit address   |
| `0x1E` | IMS         | 8-bit immediate* |
| `0x1F` | PTV         |

*Immediates are unsigned integers.