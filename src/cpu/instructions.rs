use super::Cpu;
use crate::mmu::Mmu;


impl Cpu {

    fn instructions(&self, mmu: &Mmu, instr: u8) {
        
        match instr {
            0x00 => {},  // NOP
            0x08 => {},  // LD (u16), SP
            0x10 => {},  // STOP
            0x18 => {},  // JR (unconditional)
            0x20 | 0x28 | 0x30 | 0x38 => {},  // JR (conditional)
            0x01 | 0x11 | 0x21 | 0x31 => {},  // LD r16, u16
            0x09 | 0x19 | 0x29 | 0x39 => {},  // ADD HL, r16
            0x02 | 0x12 | 0x22 | 0x32 => {},  // LD (r16), A
            0x0A | 0x1A | 0x2A | 0x3A => {},  // LD A, (r16)
            0x03 | 0x13 | 0x23 | 0x33 => {},  // INC r16
            0x0B | 0x1B | 0x2B | 0x3B => {},  // DEC r16
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {},  // INC r8
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D  => {},  // DEC r8
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E  => {},  // LD r8, u8
            0x07 | 0x0F | 0x17 | 0x1F | 0x27 | 0x2F | 0x37 | 0x3F  => {},  // (see opcode group 1 below)
            0x76 => {},  // HALT
            0x40..=0x75 | 0x77..=0x7F => {},  // LD r8, r8 (see notes: 1)
            0x80..=0xBF => {},  // ALU A, r8
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {},  // RET condition
            0xE0 => {},  // LD (FF00 + u8), A
            0xE8 => {},  // ADD SP, i8
            0xF0 => {},  // LD A, (FF00 + u8)
            0xF8 => {},  // LD HL, SP + i8
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {},  // POP r16
            0xC9 | 0xD9 | 0xE9 | 0xF9 => {},  // 0: RET, 1: RETI, 2: JP HL, 3: LD SP, HL
            0xC2 | 0xCA | 0xD2 | 0xDA => {},  // JP condition
            0xE2 => {},  // LD (FF00+C), A
            0xEA => {},  // LD (u16), A
            0xF2 => {},  // LD A, (0xFF00+C)
            0xFA => {},  // LD A, (u16)
            0xC3 => {},  // JP u16
            0xCB => {},  // CB prefix
            0xF3 => {},  // DI
            0xFB => {},  // EI
            0xC4 | 0xCC | 0xD4 | 0xDC => {},  // CALL condition
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {},  // PUSH r16
            0xCD => {},  // CALL u16
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE  => {},  // ALU a, u8
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF  => {},  // RST (Call to 00EXP000)
            _ => unreachable!()
        }

    }

    fn cb_instructions(&self, mmu: &Mmu, instr: u8) {

        match instr >> 6 {
            0 => {},  // Shifts/rotates
            1 => {},  // BIT bit, r8
            2 => {},  // RES bit, r8
            3 => {},  // SET bit, r8
            _ => unreachable!()
        }
    
    }

}
