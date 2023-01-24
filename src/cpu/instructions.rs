use super::{Cpu, registers::FlagMask, CpuState};
use crate::mmu::Mmu;


impl Cpu {

    pub fn instructions(&mut self, mmu: &mut Mmu, instr: u8) -> u8 {
        
        match instr {

            // NOP
            0x00 => { 1 },

            // LD (u16), SP
            0x08 => {
                let addr = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                mmu.write_word(addr, self.regs.sp);
                5
            },

            // STOP
            0x10 => { 1 },

            // JR (unconditional)
            0x18 => {
                //let jump_xxx = mmu.read_byte(self.regs.pc) as i32;
                let jump_len = mmu.read_byte(self.regs.pc) as i8;
                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.regs.pc = self.regs.pc.wrapping_add_signed(jump_len as i16);
                //self.regs.pc = ((self.regs.pc as u32 as i32).wrapping_add(jump_xxx)) as u16;
                3
            },

            // JR (conditional)
            0x20 | 0x28 | 0x30 | 0x38 => {
                let jump_len = mmu.read_byte(self.regs.pc) as i8;
                self.regs.pc = self.regs.pc.wrapping_add(1);
                let cond_met = match (instr >> 3) & 0x03 {
                    0 => !self.regs.get_flag(FlagMask::Z),
                    1 => self.regs.get_flag(FlagMask::Z),
                    2 => !self.regs.get_flag(FlagMask::C),
                    3 => self.regs.get_flag(FlagMask::C),
                    _ => unreachable!()
                };
                if cond_met {
                    self.regs.pc = self.regs.pc.wrapping_add_signed(jump_len as i16);
                    3
                }  else {
                    2
                }   
            },  

            // LD r16, u16
            0x01 | 0x11 | 0x21 | 0x31 => {
                let data = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                match (instr >> 4) & 0x03 {
                    0 => self.regs.set_bc(data),
                    1 => self.regs.set_de(data),
                    2 => self.regs.set_hl(data),
                    3 => self.regs.sp = data,
                    _ => unreachable!()
                }
                3
            },

            // ADD HL, r16
            0x09 | 0x19 | 0x29 | 0x39 => {
                let hl = self.regs.get_hl();
                let r16 = match (instr >> 4) & 0x3 {
                    0 => self.regs.get_bc(),
                    1 => self.regs.get_de(),
                    2 => hl,
                    3 => self.regs.sp,
                    _ => unreachable!()
                };

                let half_carry = ((hl & 0xFFF) + (r16 & 0xFFF)) >= (1 << 12);
                let (hl, carry) = hl.overflowing_add(r16);
                self.regs.set_hl(hl);

                self.regs.set_flag_val(FlagMask::N, false);
                self.regs.set_flag_val(FlagMask::H, half_carry);
                self.regs.set_flag_val(FlagMask::C, carry);
                2
            },

            // LD (r16), A
            0x02 | 0x12 | 0x22 | 0x32 => {
                let addr = match (instr >> 4) & 0x03 {
                    0 => self.regs.get_bc(),
                    1 => self.regs.get_de(),
                    2 => {
                        let hl = self.regs.get_hl();
                        self.regs.set_hl(hl.wrapping_add(1));
                        hl
                    },
                    3 => {
                        let hl = self.regs.get_hl();
                        self.regs.set_hl(hl.wrapping_sub(1));
                        hl
                    },
                    _ => unreachable!()
                };
                mmu.write_byte(addr, self.regs.a);
                2
            },

            // LD A, (r16)
            0x0A | 0x1A | 0x2A | 0x3A => {
                let addr = match (instr >> 4) & 0x03 {
                    0 => self.regs.get_bc(),
                    1 => self.regs.get_de(),
                    2 => {
                        let hl = self.regs.get_hl();
                        self.regs.set_hl(hl.wrapping_add(1));
                        hl
                    },
                    3 => {
                        let hl = self.regs.get_hl();
                        self.regs.set_hl(hl.wrapping_sub(1));
                        hl
                    },
                    _ => unreachable!()
                };
                self.regs.a = mmu.read_byte(addr);
                2
            },

            // INC r16
            0x03 | 0x13 | 0x23 | 0x33 => {
                match (instr >> 4) & 0x3 {
                    0 => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
                    1 => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
                    2 => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
                    3 => self.regs.sp = self.regs.sp.wrapping_add(1),
                    _ => unreachable!()
                }
                2
            },

            // DEC r16
            0x0B | 0x1B | 0x2B | 0x3B => {
                match (instr >> 4) & 0x3 {
                    0 => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
                    1 => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
                    2 => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
                    3 => self.regs.sp = self.regs.sp.wrapping_sub(1),
                    _ => unreachable!()
                }
                2
            },  

            // INC r8
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                let r8 = match (instr >> 3) & 0x7 {
                    0 => self.regs.b,
                    1 => self.regs.c,
                    2 => self.regs.d,
                    3 => self.regs.e,
                    4 => self.regs.h,
                    5 => self.regs.l,
                    6 => mmu.read_byte(self.regs.get_hl()),
                    7 => self.regs.a,
                    _ => unreachable!()
                };

                let res = r8.wrapping_add(1);

                self.regs.set_flag_val(FlagMask::Z, res == 0);
                self.regs.clear_flag(FlagMask::N);
                self.regs.set_flag_val(FlagMask::H, ((r8 & 0x0F) + (1 & 0x0F)) >= (1 << 4));

                match (instr >> 3) & 0x7 {
                    0 => { self.regs.b = res; 1},
                    1 => { self.regs.c = res; 1 },
                    2 => { self.regs.d = res; 1 },
                    3 => { self.regs.e = res; 1 },
                    4 => { self.regs.h = res; 1 },
                    5 => { self.regs.l = res; 1 },
                    6 => { mmu.write_byte(self.regs.get_hl(), res); 3 },
                    7 => { self.regs.a = res; 1 },
                    _ => unreachable!()
                }
            },

            // DEC r8
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D  => {
                let mut r8 = match (instr >> 3) & 0x7 {
                    0 => self.regs.b,
                    1 => self.regs.c,
                    2 => self.regs.d,
                    3 => self.regs.e,
                    4 => self.regs.h,
                    5 => self.regs.l,
                    6 => mmu.read_byte(self.regs.get_hl()),
                    7 => self.regs.a,
                    _ => unreachable!()
                };

                let res = r8.wrapping_sub(1);

                self.regs.set_flag_val(FlagMask::Z, res == 0);
                self.regs.set_flag(FlagMask::N);
                self.regs.set_flag_val(FlagMask::H, !(r8 << 4).checked_add_signed(-1 << 4).is_some());

                r8 = r8.wrapping_add(1);

                match (instr >> 3) & 0x7 {
                    0 => { self.regs.b = res; 1},
                    1 => { self.regs.c = res; 1 },
                    2 => { self.regs.d = res; 1 },
                    3 => { self.regs.e = res; 1 },
                    4 => { self.regs.h = res; 1 },
                    5 => { self.regs.l = res; 1 },
                    6 => { mmu.write_byte(self.regs.get_hl(), res); 3 },
                    7 => { self.regs.a = res; 1 },
                    _ => unreachable!()
                }
            },

            // LD r8, u8
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E  => {
                let data = mmu.read_byte(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                match (instr >> 3) & 0x7 {
                    0 => self.regs.b = data,
                    1 => self.regs.c = data,
                    2 => self.regs.d = data,
                    3 => self.regs.e = data,
                    4 => self.regs.h = data,
                    5 => self.regs.l = data,
                    6 => mmu.write_byte(self.regs.get_hl(), data),
                    7 => self.regs.a = data,
                    _ => unreachable!()
                }
                2
            },

            // RLCA
            0x07 => {
                let carry_flag = self.regs.a & 0x80 == 0x80;
                self.regs.clear_flag(FlagMask::Z);
                self.regs.clear_flag(FlagMask::N);
                self.regs.clear_flag(FlagMask::H);
                self.regs.set_flag_val(FlagMask::C, carry_flag);

                self.regs.a <<= 1;
                self.regs.a |= carry_flag as u8;
                1
            },

            // RRCA
            0x0F => {
                let carry_flag = self.regs.a & 0x01 == 0x01;
                self.regs.clear_flag(FlagMask::Z);
                self.regs.clear_flag(FlagMask::N);
                self.regs.clear_flag(FlagMask::H);
                self.regs.set_flag_val(FlagMask::C, carry_flag);

                self.regs.a >>= 1;
                self.regs.a |= (carry_flag as u8) << 7;
                1
            },

            // RLA
            0x17 => {
                let prev_carry_flag = self.regs.get_flag(FlagMask::C);
                self.regs.clear_flag(FlagMask::Z);
                self.regs.clear_flag(FlagMask::N);
                self.regs.clear_flag(FlagMask::H);
                self.regs.set_flag_val(FlagMask::C, self.regs.a & 0x80 == 0x80);

                self.regs.a <<= 1;
                self.regs.a |= prev_carry_flag as u8;
                1
            },

            // RRA
            0x1F => {
                let prev_carry_flag = self.regs.get_flag(FlagMask::C);
                self.regs.clear_flag(FlagMask::Z);
                self.regs.clear_flag(FlagMask::N);
                self.regs.clear_flag(FlagMask::H);
                self.regs.set_flag_val(FlagMask::C, self.regs.a & 0x01 == 0x01);

                self.regs.a >>= 1;
                self.regs.a |= (prev_carry_flag as u8) << 7;
                1
            },

            // DAA
            0x27 => {
                self.daa();
                1
            },

            // CPL
            0x2F => {
                self.regs.set_flag(FlagMask::N);
                self.regs.set_flag(FlagMask::H);
                self.regs.a = !self.regs.a;
                1
            },

            // SCF
            0x37 => {
                self.regs.clear_flag(FlagMask::N);
                self.regs.clear_flag(FlagMask::H);
                self.regs.set_flag(FlagMask::C);
                1
            },

            // CCF
            0x3F => {
                self.regs.clear_flag(FlagMask::N);
                self.regs.clear_flag(FlagMask::H);
                self.regs.set_flag_val(FlagMask::C, !self.regs.get_flag(FlagMask::C));
                1
            },

            // HALT
            0x76 => {
                // TODO: Implement HALT bug?
                self.state = CpuState::Halted;
                1
            },

            // LD r8, r8
            0x40 ..= 0x75 | 0x77 ..= 0x7F => {
                let r8_source = match instr & 0x7 {
                    0 => self.regs.b,
                    1 => self.regs.c,
                    2 => self.regs.d,
                    3 => self.regs.e,
                    4 => self.regs.h,
                    5 => self.regs.l,
                    6 => mmu.read_byte(self.regs.get_hl()),
                    7 => self.regs.a,
                    _ => unreachable!()
                };
                let cycles = if instr & 0x7 == 6 { 2 } else { 1 };
                match (instr >> 3) & 0x7 {
                    0 => { self.regs.b = r8_source; cycles },
                    1 => { self.regs.c = r8_source; cycles },
                    2 => { self.regs.d = r8_source; cycles },
                    3 => { self.regs.e = r8_source; cycles },
                    4 => { self.regs.h = r8_source; cycles },
                    5 => { self.regs.l = r8_source; cycles },
                    6 => { mmu.write_byte(self.regs.get_hl(), r8_source); 2 },
                    7 => { self.regs.a = r8_source; cycles },
                    _ => unreachable!()
                }
            },

            // ALU A, r8
            0x80 ..= 0xBF => {
                let r8 = match instr & 0x7 {
                    0 => self.regs.b,
                    1 => self.regs.c,
                    2 => self.regs.d,
                    3 => self.regs.e,
                    4 => self.regs.h,
                    5 => self.regs.l,
                    6 => mmu.read_byte(self.regs.get_hl()),
                    7 => self.regs.a,
                    _ => unreachable!()
                };

                match (instr >> 3) & 0x7 {
                    0 => self.add_u8(r8),
                    1 => self.adc_u8(r8),
                    2 => self.sub_u8(r8),
                    3 => self.sbc_u8(r8),
                    4 => self.and_u8(r8),
                    5 => self.xor_u8(r8),
                    6 => self.or_u8(r8),
                    7 => self.cp_u8(r8),
                    _ => unreachable!()
                }

                if (instr & 0x7) == 6 {
                    2
                } else {
                    1
                }
            },

            // RET condition
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                let cond_met = match (instr >> 3) & 0x03 {
                    0 => !self.regs.get_flag(FlagMask::Z),
                    1 => self.regs.get_flag(FlagMask::Z),
                    2 => !self.regs.get_flag(FlagMask::C),
                    3 => self.regs.get_flag(FlagMask::C),
                    _ => unreachable!()
                };
                if cond_met {
                    self.regs.pc = self.stack_pop(mmu);
                    5
                }  else {
                    2
                }   
            },

            // LD (FF00 + u8), A
            0xE0 => {
                let addr = mmu.read_byte(self.regs.pc) as u16 | 0xFF00;
                self.regs.pc = self.regs.pc.wrapping_add(1);
                mmu.write_byte(addr, self.regs.a);
                3
            },

            // ADD SP, i8
            0xE8 => {
                let r8 = mmu.read_byte(self.regs.pc) as i8 as i16 as u16;
                self.regs.pc = self.regs.pc.wrapping_add(1);

                let carry = (self.regs.sp & 0xFF) + (r8 & 0xFF) > 0xFF;
                let half_carry = (self.regs.sp & 0xF) + (r8 & 0xF) > 0xF;
                self.regs.sp = self.regs.sp.wrapping_add(r8);

                self.regs.clear_flag(FlagMask::Z);
                self.regs.clear_flag(FlagMask::N);
                self.regs.set_flag_val(FlagMask::H, half_carry);
                self.regs.set_flag_val(FlagMask::C, carry);

                4
            },

            // LD A, (FF00 + u8)
            0xF0 => {
                let addr = mmu.read_byte(self.regs.pc) as u16 | 0xFF00;
                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.regs.a = mmu.read_byte(addr);
                3
            },

            // LD HL, SP + i8
            0xF8 => {
                let r8 = mmu.read_byte(self.regs.pc) as i8 as i16 as u16;
                self.regs.pc = self.regs.pc.wrapping_add(1);

                let carry = (self.regs.sp & 0xFF) + (r8 & 0xFF) > 0xFF;
                let half_carry = (self.regs.sp & 0xF) + (r8 & 0xF) > 0xF;
                self.regs.set_hl(self.regs.sp.wrapping_add(r8));

                self.regs.clear_flag(FlagMask::Z);
                self.regs.clear_flag(FlagMask::N);
                self.regs.set_flag_val(FlagMask::H, half_carry);
                self.regs.set_flag_val(FlagMask::C, carry);

                3
            },

            // POP r16
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let data = self.stack_pop(mmu);
                match (instr >> 4) & 0x3 {
                    0 => self.regs.set_bc(data),
                    1 => self.regs.set_de(data),
                    2 => self.regs.set_hl(data),
                    3 => self.regs.set_af(data),
                    _ => unreachable!()
                }
                3
            },

            // RET
            0xC9 => {
                self.regs.pc = self.stack_pop(mmu);
                4
            },

            // RETI
            0xD9 => {
                self.regs.pc = self.stack_pop(mmu);
                // TODO: SET IME
                4
            },

            // JP HL
            0xE9 => {
                self.regs.pc = self.regs.get_hl();
                1
            },

            // LD SP, HL
            0xF9 => {
                self.regs.sp = self.regs.get_hl();
                2
            },

            // JP condition
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let jump_addr = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                let cond_met = match (instr >> 3) & 0x03 {
                    0 => !self.regs.get_flag(FlagMask::Z),
                    1 => self.regs.get_flag(FlagMask::Z),
                    2 => !self.regs.get_flag(FlagMask::C),
                    3 => self.regs.get_flag(FlagMask::C),
                    _ => unreachable!()
                };
                if cond_met {
                    self.regs.pc = jump_addr;
                    4
                } else {
                    3
                }
            },

            // LD (FF00+C), A
            0xE2 => {
                let addr = self.regs.c as u16 | 0xFF00;
                mmu.write_byte(addr, self.regs.a);
                2
            },

            // LD (u16), A
            0xEA => {
                let addr = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                mmu.write_byte(addr, self.regs.a);
                4
            },

            // LD A, (0xFF00+C)
            0xF2 => {
                let addr = self.regs.c as u16 | 0xFF00;
                self.regs.a = mmu.read_byte(addr);
                2
            },

            // LD A, (u16)
            0xFA => {
                let addr = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                self.regs.a = mmu.read_byte(addr);
                4
            },

            // JP u16
            0xC3 => {
                self.regs.pc = mmu.read_word(self.regs.pc);
                4
            },

            // CB prefix
            0xCB => {
                let instr = mmu.read_byte(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.cb_instructions(mmu, instr)
            },

            // DI
            0xF3 => {
                // TODO: clear IME
                1
            },

            // EI
            0xFB => {
                // TODO: Set IME
                // TODO: IME bug
                1
            },

            // CALL condition
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                let call_addr = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                let cond_met = match (instr >> 3) & 0x03 {
                    0 => !self.regs.get_flag(FlagMask::Z),
                    1 => self.regs.get_flag(FlagMask::Z),
                    2 => !self.regs.get_flag(FlagMask::C),
                    3 => self.regs.get_flag(FlagMask::C),
                    _ => unreachable!()
                };
                if cond_met {
                    self.stack_push(mmu, self.regs.pc);
                    self.regs.pc = call_addr;
                    6
                } else {
                    3
                }
            },

            // PUSH r16
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                match (instr >> 4) & 0x3 {
                    0 => self.stack_push(mmu, self.regs.get_bc()),
                    1 => self.stack_push(mmu, self.regs.get_de()),
                    2 => self.stack_push(mmu, self.regs.get_hl()),
                    3 => self.stack_push(mmu, self.regs.get_af()),
                    _ => unreachable!()
                }
                4
            },

            // CALL u16
            0xCD => {
                let call_addr = mmu.read_word(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(2);
                self.stack_push(mmu, self.regs.pc);
                self.regs.pc = call_addr;
                6
            },

            // ALU a, u8
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE  => {
                let r8 = mmu.read_byte(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);

                match (instr >> 3) & 0x7 {
                    0 => self.add_u8(r8),
                    1 => self.adc_u8(r8),
                    2 => self.sub_u8(r8),
                    3 => self.sbc_u8(r8),
                    4 => self.and_u8(r8),
                    5 => self.xor_u8(r8),
                    6 => self.or_u8(r8),
                    7 => self.cp_u8(r8),
                    _ => unreachable!()
                }

                2
            },

            // RST (Call to 00EXP000)
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF  => {
                let interrupt_addr = ((instr >> 3) & 0x7) * 0x08;
                self.stack_push(mmu, self.regs.pc);
                self.regs.pc = interrupt_addr as u16;
                4
            },

            // Illegal instructions
            _ => unreachable!()
        }

    }

    fn cb_instructions(&mut self, mmu: &mut Mmu, instr: u8) -> u8 {

        // println!("{:02X} {} {}", instr, (instr >> 6) & 0x3, instr & 0x7);

        match (instr >> 6) & 0x3 {

            0x0 => {

                let mut operand = match instr & 0x7 {
                    0 => self.regs.b,
                    1 => self.regs.c,
                    2 => self.regs.d,
                    3 => self.regs.e,
                    4 => self.regs.h,
                    5 => self.regs.l,
                    6 => mmu.read_byte(self.regs.get_hl()),
                    7 => self.regs.a,
                    _ => unreachable!()
                };

                match (instr >> 3) & 0x7 {
                    0 => self.rlc_u8(&mut operand),
                    1 => self.rrc_u8(&mut operand),
                    2 => self.rl_u8(&mut operand),
                    3 => self.rr_u8(&mut operand),
                    4 => self.sla_u8(&mut operand),
                    5 => self.sra_u8(&mut operand),
                    6 => self.swap_u8(&mut operand),
                    7 => self.srl_u8(&mut operand),
                    _ => unreachable!()
                }
        
                match instr & 0x7 {
                    0 => { self.regs.b = operand; 2 },
                    1 => { self.regs.c = operand; 2 },
                    2 => { self.regs.d = operand; 2 },
                    3 => { self.regs.e = operand; 2 },
                    4 => { self.regs.h = operand; 2 },
                    5 => { self.regs.l = operand; 2 },
                    6 => { mmu.write_byte(self.regs.get_hl(), operand); 4 },
                    7 => { self.regs.a = operand; 2 },
                    _ => unreachable!()
                }

            },

            // BIT bit, r8
            1 => {
                let bit = (instr >> 3) & 0x7;
                match instr & 0x7 {
                    0 => { self.bit(self.regs.b, bit); 2 },
                    1 => { self.bit(self.regs.c, bit); 2 },
                    2 => { self.bit(self.regs.d, bit); 2 },
                    3 => { self.bit(self.regs.e, bit); 2 },
                    4 => { self.bit(self.regs.h, bit); 2 },
                    5 => { self.bit(self.regs.l, bit); 2 },
                    6 => { self.bit(mmu.read_byte(self.regs.get_hl()), bit); 3 },
                    7 => { self.bit(self.regs.a, bit); 2 },
                    _ => unreachable!()
                }
            },

            // RES bit, r8
            2 => {
                let mask = !(1 << ((instr >> 3) & 0x7));
                match instr & 0x7 {
                    0 => { self.regs.b &= mask; 2 },
                    1 => { self.regs.c &= mask; 2 },
                    2 => { self.regs.d &= mask; 2 },
                    3 => { self.regs.e &= mask; 2 },
                    4 => { self.regs.h &= mask; 2 },
                    5 => { self.regs.l &= mask; 2 },
                    6 => {
                        let hl = self.regs.get_hl();
                        mmu.write_byte(hl, mmu.read_byte(hl) & mask);
                        3
                    },
                    7 => { self.regs.a &= mask; 2 },
                    _ => unreachable!()
                }
            },

            // SET bit, r8
            3 => {
                let mask = 1 << ((instr >> 3) & 0x7);
                match instr & 0x7 {
                    0 => { self.regs.b |= mask; 2 },
                    1 => { self.regs.c |= mask; 2 },
                    2 => { self.regs.d |= mask; 2 },
                    3 => { self.regs.e |= mask; 2 },
                    4 => { self.regs.h |= mask; 2 },
                    5 => { self.regs.l |= mask; 2 },
                    6 => {
                        let hl = self.regs.get_hl();
                        mmu.write_byte(hl, mmu.read_byte(hl) | mask);
                        3
                    },
                    7 => { self.regs.a |= mask; 2 },
                    _ => unreachable!()
                }
            },

            _ => unreachable!()
        }
    
    }

    fn stack_push(&mut self, mmu: &mut Mmu, data: u16) {
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        mmu.write_word(self.regs.sp, data);
    }

    fn stack_pop(&mut self, mmu: &Mmu) -> u16 {
        let word = mmu.read_word(self.regs.sp);
        self.regs.sp = self.regs.sp.wrapping_add(2);
        word
    }

    fn add_u8(&mut self, val: u8) {
        let (res, carry) = self.regs.a.overflowing_add(val);
        let half_carry = ((self.regs.a & 0x0F) + (val & 0x0F)) >= (1 << 4);
        self.regs.a = res;

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.set_flag_val(FlagMask::H, half_carry);
        self.regs.set_flag_val(FlagMask::C, carry);
    }

    fn adc_u8(&mut self, val: u8) {
        let prev_carry_flag = self.regs.get_flag(FlagMask::C) as u8;
        let carry = (self.regs.a as u16 + val as u16 + prev_carry_flag as u16) > 0xFF;
        let half_carry = ((self.regs.a & 0xF) + (val & 0xF) + prev_carry_flag) > 0xF;
        self.regs.a = self.regs.a.wrapping_add(val).wrapping_add(prev_carry_flag);

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.set_flag_val(FlagMask::H, half_carry);
        self.regs.set_flag_val(FlagMask::C, carry);
    }

    fn sub_u8(&mut self, val: u8) {
        let (res, carry) = self.regs.a.overflowing_sub(val);
        let half_carry = !(self.regs.a << 4).checked_sub(val << 4).is_some();
        self.regs.a = res;

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.set_flag(FlagMask::N);
        self.regs.set_flag_val(FlagMask::H, half_carry);
        self.regs.set_flag_val(FlagMask::C, carry);
    }

    fn sbc_u8(&mut self, val: u8) {
        let prev_carry_flag = self.regs.get_flag(FlagMask::C) as u8;
        let carry = (val as u16 + prev_carry_flag as u16) > self.regs.a as u16;
        let half_carry = ((val & 0xF) + prev_carry_flag) > self.regs.a & 0xF;
        self.regs.a = self.regs.a.wrapping_sub(val).wrapping_sub(prev_carry_flag);

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.set_flag(FlagMask::N);
        self.regs.set_flag_val(FlagMask::H, half_carry);
        self.regs.set_flag_val(FlagMask::C, carry);
    }

    fn and_u8(&mut self, val: u8) {
        self.regs.a &= val;

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.set_flag(FlagMask::H);
        self.regs.clear_flag(FlagMask::C);
    }

    fn xor_u8(&mut self, val: u8) {
        self.regs.a ^= val;

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.clear_flag(FlagMask::C);
    }

    fn or_u8(&mut self, val: u8) {
        self.regs.a |= val;

        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.clear_flag(FlagMask::C);
    }

    fn cp_u8(&mut self, val: u8) {
        self.regs.set_flag_val(FlagMask::Z, self.regs.a == val);
        self.regs.set_flag(FlagMask::N);
        self.regs.set_flag_val(FlagMask::H, self.regs.a & 0x0F < val & 0x0F);
        self.regs.set_flag_val(FlagMask::C, self.regs.a < val);
    }

    fn daa(&mut self) {

        if !self.regs.get_flag(FlagMask::N) {
            
            if self.regs.get_flag(FlagMask::C) || self.regs.a > 0x99 {
                self.regs.a += 0x60;
                self.regs.set_flag_val(FlagMask::C, true);
            }
            if self.regs.get_flag(FlagMask::H) || (self.regs.a & 0x0F) > 0x09 {
                self.regs.a += 0x06;
            }

        } else {

            if self.regs.get_flag(FlagMask::C) {
                self.regs.a -= 0x60;
            }
            if self.regs.get_flag(FlagMask::H) {
                self.regs.a -= 0x06;
            }
            
        }
    
        self.regs.set_flag_val(FlagMask::Z, self.regs.a == 0);
        self.regs.clear_flag(FlagMask::H);
    }

    fn rlc_u8(&mut self, reg: &mut u8) {
        let carry_flag = *reg & 0x80 == 0x80;
        *reg <<= 1;
        *reg |= carry_flag as u8;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn rrc_u8(&mut self, reg: &mut u8) {
        let carry_flag = *reg & 0x01 == 0x01;
        *reg >>= 1;
        *reg |= (carry_flag as u8) << 7;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn rl_u8(&mut self, reg: &mut u8) {
        let prev_carry_flag = self.regs.get_flag(FlagMask::C);
        let carry_flag = *reg & 0x80 == 0x80;
        *reg <<= 1;
        *reg |= prev_carry_flag as u8;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn rr_u8(&mut self, reg: &mut u8) {
        let prev_carry_flag = self.regs.get_flag(FlagMask::C);
        let carry_flag = *reg & 0x01 == 0x01;
        *reg >>= 1;
        *reg |= (prev_carry_flag as u8) << 7;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn sla_u8(&mut self, reg: &mut u8) {
        let carry_flag = *reg & 0x80 == 0x80;
        *reg <<= 1;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn sra_u8(&mut self, reg: &mut u8) {
        let carry_flag = *reg & 0x01 == 0x01;
        *reg = ((*reg as i8) >> 1) as u8;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn swap_u8(&mut self, reg: &mut u8) {
        *reg = ((*reg & 0x0F) << 4) | ((*reg & 0xF0) >> 4);

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.clear_flag(FlagMask::C);
    }

    fn srl_u8(&mut self, reg: &mut u8) {
        let carry_flag = *reg & 0x01 == 0x01;
        *reg >>= 1;

        self.regs.set_flag_val(FlagMask::Z, *reg == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.clear_flag(FlagMask::H);
        self.regs.set_flag_val(FlagMask::C, carry_flag);
    }

    fn bit(&mut self, val: u8, bit: u8) {
        self.regs.set_flag_val(FlagMask::Z, val & (1 << bit) == 0);
        self.regs.clear_flag(FlagMask::N);
        self.regs.set_flag(FlagMask::H);
    }

}
