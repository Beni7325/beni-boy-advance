use crate::cpu::Arm7Tdmi;
use crate::cpu::alu::{self, ShiftOp, MovCmpAddSubOp, AluOp};
use crate::cpu::decode::*;
use crate::fill_dispatch;

const SIZE_THUMB_DISPATCH: usize = 1 << 10; // We use 10 bits to decode thumb instructions 

impl Arm7Tdmi {

    pub const THUMB_DISPATCH: [fn(&mut Self, u16); SIZE_THUMB_DISPATCH] = fill_dispatch!(
        size: SIZE_THUMB_DISPATCH,
        default: Arm7Tdmi::thumb_noop,
        "00011....." => Arm7Tdmi::thumb_add_sub,              // ADD, SUB
        "000......." => Arm7Tdmi::thumb_lsl_lsr_asr,          // LSL, LSR, ASR
        "001......." => Arm7Tdmi::thumb_mov_cmp_add_sub_imm,  // MOV, CMP, ADD, SUB immediate
        "010000...." => Arm7Tdmi::thumb_alu_operations,       // ALU operations
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
            alu::sub(rs, operand)
        } else {
            alu::add(rs, operand)
        };

        self.registers.write_register(rd_idx, result);

        self.registers.set_negative_flag((result as i32).is_negative());
        self.registers.set_zero_flag(result == 0);
        self.registers.set_carry_flag(carry);
        self.registers.set_overflow_flag(overflow);
    }

    fn thumb_lsl_lsr_asr(&mut self, instr: u16) {

        let instr = instr as u32;
        let shift_operation = extract_bits(instr, 12, 2).into();
        let shift  = extract_bits(instr, 10, 5) as u8;
        let rs_idx = extract_bits(instr, 5, 3);
        let rd_idx = extract_bits(instr, 2, 3);

        let rs = self.registers.read_register(rs_idx);

        let (result, carry): (u32, Option<bool>) = match shift_operation {
            ShiftOp::Lsl => alu::lsl(rs, shift),
            ShiftOp::Lsr => alu::lsr(rs, shift, true),
            ShiftOp::Asr => alu::asr(rs, shift, true),
            ShiftOp::Ror => unreachable!()
        };
        
        self.registers.write_register(rd_idx, result);

        if let Some(c) = carry {
            self.registers.set_carry_flag(c);
        }
        self.registers.set_negative_flag((result as i32).is_negative());
        self.registers.set_zero_flag(result == 0);
    }

    fn thumb_mov_cmp_add_sub_imm(&mut self, instr: u16) {
        let instr = instr as u32;
        let op = extract_bits(instr, 12, 2).into();
        let rd_idx = extract_bits(instr, 10, 3);
        let imm = extract_bits(instr, 7, 8);
        let rd = self.registers.read_register(rd_idx);

        let with_carry_overflow = |(r, c, v)| (r, Some((c, v)));

        let (result, carry_overflow): (u32, Option<(bool, bool)>) = match op {
            MovCmpAddSubOp::Mov                       => (imm, None),
            MovCmpAddSubOp::Cmp | MovCmpAddSubOp::Sub => with_carry_overflow(alu::sub(rd, imm)),
            MovCmpAddSubOp::Add                       => with_carry_overflow(alu::add(rd, imm))
        };

        if !matches!(op, MovCmpAddSubOp::Cmp) {
            self.registers.write_register(rd_idx, result);
        }

        self.registers.set_negative_flag((result as i32).is_negative());
        self.registers.set_zero_flag(result == 0);
        if let Some((carry, overflow)) = carry_overflow {
            self.registers.set_carry_flag(carry);
            self.registers.set_overflow_flag(overflow);
        }
    }

    fn thumb_alu_operations(&mut self, instr: u16) {
        let instr = instr as u32;
        let alu_op = extract_bits(instr, 9, 4).into();
        let rs_idx = extract_bits(instr, 5, 3);
        let rd_idx = extract_bits(instr, 2, 3);

        let rs = self.registers.read_register(rs_idx);
        let rd = self.registers.read_register(rd_idx);

        let with_carry_overflow = |(r, c, v)| (r, Some(c), Some(v));
        let with_carry = |(r, c)| (r, c, None);

        let (result, carry, overflow): (u32, Option<bool>, Option<bool>) = match alu_op {
            // Logical
            AluOp::And => (rd &  rs, None, None),
            AluOp::Eor => (rd ^  rs, None, None),
            AluOp::Tst => (rd &  rs, None, None),
            AluOp::Orr => (rd |  rs, None, None),
            AluOp::Bic => (rd & !rs, None, None),
            AluOp::Mvn => (     !rs, None, None),

            // Shifts
            AluOp::Lsl => with_carry(alu::lsl(rd, rs as u8)),
            AluOp::Lsr => with_carry(alu::lsr(rd, rs as u8, false)),
            AluOp::Asr => with_carry(alu::asr(rd, rs as u8, false)),
            AluOp::Ror => with_carry(alu::ror(rd, rs as u8)),

            // Arithmetic
            AluOp::Adc => with_carry_overflow(alu::adc(rd, rs,  self.registers.get_carry_flag())),
            AluOp::Sbc => with_carry_overflow(alu::sbc(rd, rs, !self.registers.get_carry_flag())),
            AluOp::Neg => with_carry_overflow(alu::sub( 0, rs)),
            AluOp::Cmp => with_carry_overflow(alu::sub(rd, rs)),
            AluOp::Cmn => with_carry_overflow(alu::add(rd, rs)),
            AluOp::Mul => (alu::mul(rd, rs), None, None)
        };

        if !matches!(alu_op, AluOp::Tst | AluOp::Cmp | AluOp::Cmn) {
            self.registers.write_register(rd_idx, result);
        }

        if let Some(c) = carry {
            self.registers.set_carry_flag(c);
        }
        if let Some(v) = overflow {
            self.registers.set_overflow_flag(v);
        }
        self.registers.set_negative_flag((result as i32).is_negative());
        self.registers.set_zero_flag(result == 0);
    }
}
