use crate::cpu::Arm7Tdmi;
use crate::cpu::decode::*;
use crate::fill_dispatch;

const SIZE_THUMB_DISPATCH: usize = 1 << 10; // We use 10 bits to decode thumb instructions 

impl Arm7Tdmi {

    pub const THUMB_DISPATCH: [fn(&mut Self, u16); SIZE_THUMB_DISPATCH] = fill_dispatch!(
        size: SIZE_THUMB_DISPATCH,
        default: Arm7Tdmi::thumb_noop,
        "00011....." => Arm7Tdmi::thumb_add_sub,     // ADD, SUB
        "000......." => Arm7Tdmi::thumb_lsl_lsr_asr  // LSL, LSR, ASR
    );

    fn thumb_noop(&mut self, instr: u16) {
        println!("[NOOP]: Thumb instruction 0x{:04X} not implemented or illegal", instr);
    }

    fn thumb_add_sub(&mut self, instr: u16) {
        let instr = instr as u32;
        let use_immediate = extract_bits(instr, 10, 1) != 0;
        let subtract = extract_bits(instr,  9, 1) != 0;
        let rs_idx = extract_bits(instr,  5, 3);
        let rd_idx = extract_bits(instr,  2, 3);

        let operand = {
            let raw = extract_bits(instr, 8, 3);
            if use_immediate { raw } else { self.registers.read_register(raw) }
        };

        let rs = self.registers.read_register(rs_idx);

        let (result, carry, overflow) = if subtract {
            let (res, borrow) = rs.overflowing_sub(operand);
            let (_, overflow) = (rs as i32).overflowing_sub(operand as i32);
            (res, !borrow, overflow)
        } else {
            let (res, carry) = rs.overflowing_add(operand);
            let (_, overflow) = (rs as i32).overflowing_add(operand as i32);
            (res, carry, overflow)
        };

        self.registers.write_register(rd_idx, result);

        self.registers.set_negative_flag((result as i32).is_negative());
        self.registers.set_zero_flag(result == 0);
        self.registers.set_carry_flag(carry);
        self.registers.set_overflow_flag(overflow);
    }

    fn thumb_lsl_lsr_asr(&mut self, instr: u16) {
        #[derive(Clone, Copy)]
        enum ShiftOp { Lsl, Lsr, Asr }

        const MSB: u32 = 1 << (u32::BITS - 1);

        let instr = instr as u32;
        let op = match extract_bits(instr, 12, 2) {
            0 => ShiftOp::Lsl,
            1 => ShiftOp::Lsr,
            2 => ShiftOp::Asr,
            _ => unreachable!(),
        };
        let shift  = extract_bits(instr, 10, 5);
        let rs_idx = extract_bits(instr, 5, 3);
        let rd_idx = extract_bits(instr, 2, 3);

        let rs = self.registers.read_register(rs_idx);

        // LSL with a shift amount of 0 leaves the register and carry flag unchanged
        let (result, carry): (u32, Option<bool>) = if shift == 0 {
            match op {
                ShiftOp::Lsl => (rs, None),
                ShiftOp::Lsr => (0, Some(rs & MSB != 0)),
                ShiftOp::Asr => {
                    let sign = (rs as i32).is_negative();
                    (if sign { u32::MAX } else { 0 }, Some(sign))
                }
            }
        } else {
            match op {
                ShiftOp::Lsl => (rs << shift, Some(rs & (MSB >> (shift - 1)) != 0)),
                ShiftOp::Lsr => (rs >> shift, Some(rs & (1 << (shift - 1)) != 0)),
                ShiftOp::Asr => ((rs as i32 >> shift) as u32, Some(rs & (1 << (shift - 1)) != 0)),
            }
        };
        
        self.registers.write_register(rd_idx, result);

        if let Some(c) = carry {
            self.registers.set_carry_flag(c);
        }
        self.registers.set_negative_flag((result as i32).is_negative());
        self.registers.set_zero_flag(result == 0);
    }
}
