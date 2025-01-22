use macroquad::window::request_new_screen_size;
use crate::emulator::Emulator;
use crate::emulator::instruction::Instruction;

mod emulator;

#[cfg(test)]
mod tests;

#[macroquad::main("Emulator")]
async fn main() {
    request_new_screen_size(512.0, 256.0);

    let mut code: Vec<u8> = Vec::new();
    code.push(Instruction::IMS.into());
    code.push(255);
    code.push(Instruction::POP.into());
    code.push(241);
    code.push(0);
    code.push(Instruction::JMP.into());
    code.push(16);
    code.push(5);

    let mut emulator = Emulator::new(code);
    emulator.run().await;
}
