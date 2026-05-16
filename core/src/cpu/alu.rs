const MSB: u32 = 1 << (u32::BITS - 1);
const LSB: u32 = 1;

#[derive(Clone, Copy)]
pub enum ShiftOp {
    Lsl,
    Lsr,
    Asr,
    Ror
}

impl From<u32> for ShiftOp {
    fn from(shift_op: u32) -> Self {
        match shift_op {
            0 => ShiftOp::Lsl,
            1 => ShiftOp::Lsr,
            2 => ShiftOp::Asr,
            3 => ShiftOp::Ror,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MovCmpAddSubOp {
    Mov,
    Cmp,
    Add,
    Sub
}

impl From<u32> for MovCmpAddSubOp {
    fn from(op: u32) -> Self {
        match op {
            0 => MovCmpAddSubOp::Mov,
            1 => MovCmpAddSubOp::Cmp,
            2 => MovCmpAddSubOp::Add,
            3 => MovCmpAddSubOp::Sub,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AluOp {
    And,
    Eor,
    Lsl,
    Lsr,
    Asr,
    Adc,
    Sbc,
    Ror,
    Tst,
    Neg,
    Cmp,
    Cmn,
    Orr,
    Mul,
    Bic,
    Mvn
}

impl From<u32> for AluOp {
    fn from(alu_op: u32) -> Self {
        match alu_op {
             0 => AluOp::And,  1 => AluOp::Eor,  2 => AluOp::Lsl,  3 => AluOp::Lsr,
             4 => AluOp::Asr,  5 => AluOp::Adc,  6 => AluOp::Sbc,  7 => AluOp::Ror,
             8 => AluOp::Tst,  9 => AluOp::Neg, 10 => AluOp::Cmp, 11 => AluOp::Cmn,
            12 => AluOp::Orr, 13 => AluOp::Mul, 14 => AluOp::Bic, 15 => AluOp::Mvn,
            _ => unreachable!(),
        }
    }
}

pub fn lsl(operand: u32, shift: u8) -> (u32, Option<bool>) {
    match shift {
        0      => (operand, None),
        1..=31 => (operand << shift, Some(operand & (MSB >> (shift - 1)) != 0)),
        32     => (0, Some(operand & LSB != 0)),
        _      => (0,Some(false)),
    }
}

pub fn lsr(operand: u32, shift: u8, imm_shift: bool) -> (u32, Option<bool>) {
    match shift {
        0 if !imm_shift => (operand, None),
        1..=31          => (operand >> shift, Some(operand & (1 << (shift - 1)) != 0)),
        0 | 32          => (0, Some(operand & MSB != 0)), // imm shift of 0 encodes shift by 32
        _               => (0, Some(false)),
    }
}

pub fn asr(operand: u32, shift: u8, imm_shift: bool) -> (u32, Option<bool>) {
    match shift {
        0 if !imm_shift => (operand, None),
        1..=31 => (
            (operand as i32 >> shift) as u32,
            Some(operand & (1 << (shift - 1)) != 0),
        ),
        _ => {
            // imm shift of 0 encodes shift by 32
            let sign = (operand as i32).is_negative();
            (if sign { u32::MAX } else { 0 }, Some(sign))
        }
    }
}

pub fn ror(operand: u32, shift: u8) -> (u32, Option<bool>) {
    match (shift % 32, shift == 0) {
        (0, true) => (operand, None),
        (0, false) => (operand, Some(operand & MSB != 0)),
        (shift @ 1..=31, _) => (operand.rotate_right(shift as u32), Some(operand & (1 << (shift - 1)) != 0)),
        _ => unreachable!()
    }
}

pub fn add(operand1: u32, operand2: u32) -> (u32, bool, bool) {
    let (result, carry) = operand1.overflowing_add(operand2);
    let (_, overflow) = (operand1 as i32).overflowing_add(operand2 as i32);
    (result, carry, overflow)
}

pub fn adc(operand1: u32, operand2: u32, carry: bool) -> (u32, bool, bool) {
    let (result, carry) = operand1.carrying_add(operand2, carry);
    let overflow = (!(operand1 ^ operand2) & (operand1 ^ result)) & MSB != 0;
    (result, carry, overflow)
}

pub fn sub(operand1: u32, operand2: u32) -> (u32, bool, bool) {
    let (result, borrow) = operand1.overflowing_sub(operand2);
    let (_, overflow)  = (operand1 as i32).overflowing_sub(operand2 as i32);
    (result, !borrow, overflow)
}

pub fn sbc(operand1: u32, operand2: u32, carry: bool) -> (u32, bool, bool) {
    let (result, borrow) = operand1.borrowing_sub(operand2, carry);
    let overflow = ((operand1 ^ operand2) & (operand1 ^ result)) & MSB != 0;
    (result, !borrow, overflow)
}

pub fn mul(operand1: u32, operand2: u32) -> u32 {
    // According to the spec the value of the carry flag after a MUL is undefined so for now we dont calculate it
    operand1.overflowing_mul(operand2).0
}
