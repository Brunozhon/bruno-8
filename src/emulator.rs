use crate::emulator::instruction::Instruction;
use crate::emulator::memory::Memory;
use macroquad::prelude::*;

const DEBUG: bool = true;

pub mod instruction;
pub mod memory;
pub mod fonts;

pub struct Emulator {
    memory: Memory,
    running: bool,
}

impl Emulator {
    pub fn new(program: Vec<u8>) -> Emulator {
        let mut memory = Memory::new();
        memory.write_vec(4096, program);
        memory.write(65534, 16);
        memory.write_vec(0, Vec::from(fonts::FONTS));
        Emulator {
            memory,
            running: true,
        }
    }

    pub async fn run(&mut self) {
        while self.running {
            clear_background(BLACK);
            self.run_instruction();

            for i in 0..512 {
                let x = i % 32 * 16;
                let y = i / 32 * 16;

                let color_num = self.memory.peek(61696 + i);
                let r = (color_num & 0b11100000) >> 5;
                let g = (color_num & 0b00011100) >> 2;
                let b = color_num & 0b00000011;

                let color = Color::new(r as f32 / 7f32,
                                       g as f32 / 7f32, b as f32 / 3f32, 1.0);

                draw_rectangle(x as f32, y as f32, 4.0, 4.0, color);
            }

            next_frame().await;
        }
    }

    pub fn run_unwindowed(&mut self) {
        while self.running {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let mut pc: u16 = ((self.memory.peek(65534) as u16) << 8) + self.memory.peek(65535) as u16;
        let mut sp = self.memory.peek(65533);
        let inst = self.memory.peek(pc);

        if DEBUG {
            for i in 0..sp {
                print!("[{:03}] ", self.memory.peek(61440 + i as u16));
            }
            println!();
        }

        match Instruction::from(inst) {
            Instruction::HLT => {
                if DEBUG {
                    println!("HLT");
                }
                self.running = false;
            },
            Instruction::ADD => {
                if DEBUG {
                    println!("ADD");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a + b);
            }
            Instruction::SUB => {
                if DEBUG {
                    println!("SUB");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a - b);
            }
            Instruction::MUL => {
                if DEBUG {
                    println!("MUL");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a * b);
            }
            Instruction::DIV => {
                if DEBUG {
                    println!("DIV");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a / b);
            }
            Instruction::PUSH => {
                let addr = self.memory.peek2(pc + 1);
                let value = self.memory.peek(addr);
                if DEBUG {
                    println!("PUSH value {} at address {}", value, addr);
                }
                self.push_stack(value);
                sp += 1;
                pc += 2;
            }
            Instruction::POP => {
                let value = self.pop_stack();
                let addr = self.memory.peek2(pc + 1);
                if DEBUG {
                    println!("POP value {} to address {}", value, addr);
                }
                self.memory.poke(addr, value);
                pc += 2;
                sp -= 1;
            }
            Instruction::AND => {
                if DEBUG {
                    println!("AND");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a & b);
            }
            Instruction::NAND => {
                if DEBUG {
                    println!("NAND");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(!(a & b));
            }
            Instruction::OR => {
                if DEBUG {
                    println!("OR");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a | b);
            }
            Instruction::XOR => {
                if DEBUG {
                    println!("XOR");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(a ^ b);
            }
            Instruction::NOR => {
                if DEBUG {
                    println!("DIV");
                }
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.push_stack(!(a | b));
            }
            Instruction::NOT => {}
            Instruction::INC => {}
            Instruction::DEC => {}
            Instruction::NOP => {}
            Instruction::PSP => {}
            Instruction::SSP => {}
            Instruction::EQ => {}
            Instruction::GT => {}
            Instruction::LT => {}
            Instruction::GEQ => {}
            Instruction::LEQ => {}
            Instruction::NEQ => {}
            Instruction::JMP => {
                let addr = self.memory.peek2(pc + 1);
                pc = addr - 1;
                if DEBUG {
                    println!("JMP to {}", addr);
                }
            }
            Instruction::JNZ => {}
            Instruction::JEZ => {}
            Instruction::IMS => {
                let value = self.memory.peek(pc + 1);
                assert_ne!(sp, 255, "Stack overflow");
                if DEBUG {
                    println!("PUSH value {}", value);
                }
                self.memory.write(61440 + sp as u16, value);
                sp += 1;
                pc += 1;
                self.memory.poke(65533, sp);
            }
            Instruction::PTV => {
                if DEBUG {
                    println!("PTV");
                }
                if sp > 0 {
                    sp -= 1;
                }
                self.memory.poke(65533, sp);
            }
        }

        pc += 1;
        self.memory.poke(65534, (pc >> 8) as u8);
        self.memory.poke(65535, (pc & 255) as u8);
    }

    pub fn pop_stack(&mut self) -> u8 {
        let mut sp = self.memory.peek(65533);
        assert!(sp > 0, "Stack underflow");
        sp -= 1;
        self.memory.poke(65533, sp);
        self.memory.peek(61440 + sp as u16)
    }

    pub fn push_stack(&mut self, value: u8) {
        let mut sp = self.memory.peek(65533);
        assert_ne!(sp, 255, "Stack overflow");
        self.memory.poke(61440 + sp as u16, value);
        sp += 1;
        self.memory.poke(65533, sp);
    }
}