use crate::cpu::decode::extract_bits;
use crate::cpu::registers::Registers;

#[derive(Default)]
pub struct Arm7Tdmi {
    pub registers: Registers,
}

impl Arm7Tdmi {
    pub fn execute_thumb(&mut self, opcode: u16) {
        self.registers.increase_pc(2);

        // We only look at bits 15-6 to decode THUMB instructions
        let important_bits = extract_bits(opcode as u32, 15, 10) as usize;
        (Arm7Tdmi::THUMB_DISPATCH[important_bits & 0x3FF])(self, opcode);
    }
}
