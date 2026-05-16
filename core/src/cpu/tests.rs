use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use pretty_assertions::assert_eq;
use crate::cpu::Arm7Tdmi;
use crate::cpu::decode::extract_bits;
use crate::cpu::registers::Registers;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CpuState {
    #[serde(rename = "R")]     pub r:     [u32; 16],
    #[serde(rename = "R_fiq")] pub r_fiq: [u32; 7],
    #[serde(rename = "R_svc")] pub r_svc: [u32; 2],
    #[serde(rename = "R_abt")] pub r_abt: [u32; 2],
    #[serde(rename = "R_irq")] pub r_irq: [u32; 2],
    #[serde(rename = "R_und")] pub r_und: [u32; 2],
    #[serde(rename = "CPSR")]  pub cpsr:  u32,
    #[serde(rename = "SPSR")]  pub spsr:  [u32; 5],
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub kind:   u8,
    pub size:   u8,
    pub addr:   u64,
    pub data:   u64,
    pub cycle:  u32,
    pub access: u8,
}

#[derive(Debug, Deserialize)]
pub struct TestFixture {
    pub initial:      CpuState,
    #[serde(rename = "final")]
    pub final_state:  CpuState,
    pub transactions: Vec<Transaction>,
    pub opcode:       u32,
    pub base_addr:    u64,
}

impl From<CpuState> for Registers {
    fn from(state: CpuState) -> Self {
        let mut regs = Registers::default();

        regs.gp_regs[0..8].copy_from_slice(&state.r[0..8]);
        regs.gp_regs[15] = state.r[15];

        regs.r8_r12_usr.copy_from_slice(&state.r[8..13]);
        regs.r8_r12_fiq.copy_from_slice(&state.r_fiq[0..5]);

        regs.sp_lr_usr.copy_from_slice(&state.r[13..15]);
        regs.sp_lr_fiq.copy_from_slice(&state.r_fiq[5..7]);
        regs.sp_lr_svc = state.r_svc;
        regs.sp_lr_abt = state.r_abt;
        regs.sp_lr_irq = state.r_irq;
        regs.sp_lr_und = state.r_und;

        regs.cpsr = state.cpsr.into();

        regs.spsr_fiq = state.spsr[0].into();
        regs.spsr_svc = state.spsr[1].into();
        regs.spsr_abt = state.spsr[2].into();
        regs.spsr_irq = state.spsr[3].into();
        regs.spsr_und = state.spsr[4].into();
        

        regs.load_banked_regs(regs.cpsr.mode);

        regs
    }
}

impl From<Registers> for CpuState {
    fn from(regs: Registers) -> Self {
        let mut r = [0u32; 16];
        r[0..8].copy_from_slice(&regs.gp_regs[0..8]);
        r[8..13].copy_from_slice(&regs.r8_r12_usr[0..5]);
        r[13..15].copy_from_slice(&regs.sp_lr_usr);
        r[15] = regs.gp_regs[15];

        let mut r_fiq = [0u32; 7];
        r_fiq[0..5].copy_from_slice(&regs.r8_r12_fiq[0..5]);
        r_fiq[5..7].copy_from_slice(&regs.sp_lr_fiq);

        CpuState {
            r,
            r_fiq,
            r_svc: regs.sp_lr_svc,
            r_abt: regs.sp_lr_abt,
            r_irq: regs.sp_lr_irq,
            r_und: regs.sp_lr_und,
            cpsr:  regs.cpsr.into(),
            spsr: [
                regs.spsr_fiq.into(),
                 regs.spsr_svc.into(),
                 regs.spsr_abt.into(),
                 regs.spsr_irq.into(),
                 regs.spsr_und.into()
             ],
        }
    }
}

fn load_test_fixtures<P: AsRef<Path>>(relative_path: P) -> Vec<TestFixture> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    let file = File::open(&path)
        .unwrap_or_else(|_| panic!("Fixture file not found at {path:?}"));
    serde_json::from_reader(BufReader::new(file))
        .unwrap_or_else(|_| panic!("Failed to parse JSON fixture at {path:?}"))
}

fn run_thumb_fixture_test(fixture_path: &str) {

    let mut cpu = Arm7Tdmi::default();

    for (i, fixture) in load_test_fixtures(fixture_path).iter().enumerate() {

        cpu.registers = fixture.initial.clone().into();
        cpu.execute_thumb(fixture.opcode as u16);
        cpu.registers.save_banked_regs();

        assert_eq!(
            fixture.final_state,
            CpuState::from(cpu.registers),
            "Test case {i} failed",
        );
    }
}

fn run_thumb_fixture_test_no_mul_carry(fixture_path: &str, high_bit: u8, n_bits: u8, mul_opcode: u8) {

    let mut cpu = Arm7Tdmi::default();

    for (i, fixture) in load_test_fixtures(fixture_path).iter_mut().enumerate() {
        
        cpu.registers = fixture.initial.clone().into();
        cpu.execute_thumb(fixture.opcode as u16);
        cpu.registers.save_banked_regs();

        if extract_bits(fixture.opcode, high_bit, n_bits) == mul_opcode as u32 {
            // Sets the 29th bit of the expected cpsr (carry) to the one of the initial state
            // This is done as the carry flag after a mul is undefined per the spec and for now we just dont change it
            let bit_29_mask = 1 << 29;
            fixture.final_state.cpsr = (fixture.final_state.cpsr & !bit_29_mask) | (fixture.initial.cpsr & bit_29_mask);
        }

        assert_eq!(
            fixture.final_state,
            CpuState::from(cpu.registers),
            "Test case {i} failed",
        );
    }
}

#[test]
fn test_thumb_add_sub() {
    run_thumb_fixture_test("tests/single-step-tests/thumb_add_sub.json");
}

#[test]
fn test_thumb_lsl_lsr_asr() {
    run_thumb_fixture_test("tests/single-step-tests/thumb_lsl_lsr_asr.json");
}

#[test]
fn test_thumb_mov_cmp_add_sub_imm() {
    run_thumb_fixture_test("tests/single-step-tests/thumb_mov_cmp_add_sub.json");
}

#[test]
fn test_thumb_alu_operations() {
    run_thumb_fixture_test_no_mul_carry("tests/single-step-tests/thumb_data_proc.json", 9, 4, 0b1101);
}
