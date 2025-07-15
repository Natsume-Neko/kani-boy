mod instruction;
mod memory;
mod registers;
use std::ops::{Shl, ShlAssign};

use instruction::{ArithmeticTarget, Instruction};
use registers::Registers;

use crate::cpu::{instruction::ArithmeticTarget16Bit, memory::MemoryBus};

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::Adc(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::Sub(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::Sbc(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::And(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::Or(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::Xor(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.xor(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::Cp(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    self.sub(value);
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    self.sub(value);
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    self.sub(value);
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    self.sub(value);
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    self.sub(value);
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    self.sub(value);
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    self.sub(value);
                }
            },
            Instruction::Inc(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.inc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.inc(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.inc(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.inc(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.inc(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.inc(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.inc(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Dec(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.dec(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.dec(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.dec(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.dec(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.dec(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.dec(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.dec(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::AddHL(target) => match target {
                ArithmeticTarget16Bit::BC => {
                    let value = self.registers.get_bc();
                    let new_value = self.addhl(value);
                    self.registers.set_bc(new_value);
                }
                ArithmeticTarget16Bit::DE => {
                    let value = self.registers.get_de();
                    let new_value = self.addhl(value);
                    self.registers.set_de(new_value);
                }
                ArithmeticTarget16Bit::HL => {
                    let value = self.registers.get_hl();
                    let new_value = self.addhl(value);
                    self.registers.set_hl(new_value);
                }
            },
            Instruction::Inc16Bit(target) => match target {
                ArithmeticTarget16Bit::BC => {
                    let value = self.registers.get_bc();
                    let new_value = self.inc_16bit(value);
                    self.registers.set_bc(new_value);
                }
                ArithmeticTarget16Bit::DE => {
                    let value = self.registers.get_de();
                    let new_value = self.inc_16bit(value);
                    self.registers.set_de(new_value);
                }
                ArithmeticTarget16Bit::HL => {
                    let value = self.registers.get_hl();
                    let new_value = self.inc_16bit(value);
                    self.registers.set_hl(new_value);
                }
            },
            Instruction::Dec16Bit(target) => match target {
                ArithmeticTarget16Bit::BC => {
                    let value = self.registers.get_bc();
                    let new_value = self.dec_16bit(value);
                    self.registers.set_bc(new_value);
                }
                ArithmeticTarget16Bit::DE => {
                    let value = self.registers.get_de();
                    let new_value = self.dec_16bit(value);
                    self.registers.set_de(new_value);
                }
                ArithmeticTarget16Bit::HL => {
                    let value = self.registers.get_hl();
                    let new_value = self.dec_16bit(value);
                    self.registers.set_hl(new_value);
                }
            },
            Instruction::Swap(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.swap(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.swap(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.swap(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.swap(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.swap(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.swap(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.swap(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Cpl => {
                let value = self.registers.a;
                let new_value = self.cpl(value);
                self.registers.a = new_value;
            }
            Instruction::Ccf => {
                self.ccf();
            }
            Instruction::Scf => {
                self.scf();
            }
            Instruction::Rlca => {
                let value = self.registers.a;
                let new_value = self.rlca(value);
                self.registers.a = new_value;
            }
            Instruction::Rla => {
                let value = self.registers.a;
                let new_value = self.rla(value);
                self.registers.a = new_value;
            }
            Instruction::Rrca => {
                let value = self.registers.a;
                let new_value = self.rrca(value);
                self.registers.a = new_value;
            }
            Instruction::Rra => {
                let value = self.registers.a;
                let new_value = self.rra(value);
                self.registers.a = new_value;
            }
            Instruction::Rlc(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.rlc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rlc(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rlc(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rlc(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rlc(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rlc(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rlc(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Rl(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.rl(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rl(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rl(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rl(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rl(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rl(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rl(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Rrc(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.rrc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rrc(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rrc(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rrc(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rrc(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rrc(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rrc(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Rr(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.rr(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rr(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rr(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rr(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rr(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rr(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rr(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Sla(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sla(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sla(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sla(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sla(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sla(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sla(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sla(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Sra(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sra(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sra(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sra(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sra(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sra(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sra(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sra(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Srl(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.srl(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.srl(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.srl(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.srl(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.srl(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.srl(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.srl(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Bit((b, target)) => match target {
                ArithmeticTarget::A => self.bit(b, self.registers.a),
                ArithmeticTarget::B => self.bit(b, self.registers.b),
                ArithmeticTarget::C => self.bit(b, self.registers.c),
                ArithmeticTarget::D => self.bit(b, self.registers.d),
                ArithmeticTarget::E => self.bit(b, self.registers.e),
                ArithmeticTarget::H => self.bit(b, self.registers.h),
                ArithmeticTarget::L => self.bit(b, self.registers.l),
            },
            Instruction::Set((b, target)) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.set(b, value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.set(b, value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.set(b, value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.set(b, value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.set(b, value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.set(b, value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.set(b, value);
                    self.registers.l = new_value;
                }
            },
            Instruction::Res((b, target)) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.res(b, value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.res(b, value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.res(b, value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.res(b, value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.res(b, value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.res(b, value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.res(b, value);
                    self.registers.l = new_value;
                }
            },
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, is_overflow) = self.registers.a.overflowing_add(value);

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = is_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn adc(&mut self, value: u8) -> u8 {
        let carry = self.registers.f.carry as u8;
        let (a_with_carry, is_overflow_carry) = self.registers.a.overflowing_add(carry);
        let (new_value, is_overflow_add) = a_with_carry.overflowing_add(value);

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = is_overflow_carry | is_overflow_add;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + 0x1 > 0xF;

        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, is_borrow) = self.registers.a.overflowing_sub(value);

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = true;
        self.registers.f.carry = is_borrow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);

        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let carry = self.registers.f.carry as u8;
        let (a_with_carry, is_borrow_carry) = self.registers.a.overflowing_sub(carry);
        let (new_value, is_borrow_sub) = a_with_carry.overflowing_sub(value);

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = true;
        self.registers.f.carry = is_borrow_carry | is_borrow_sub;
        self.registers.f.half_carry = (self.registers.a & 0xF) < ((value & 0xF) + 0x1);

        new_value
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;

        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;

        // Set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn inc(&mut self, value: u8) -> u8 {
        let (new_value, _) = value.overflowing_add(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;

        new_value
    }

    fn dec(&mut self, value: u8) -> u8 {
        let (new_value, _) = value.overflowing_sub(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = true;
        self.registers.f.half_carry = (value & 0xF) < 1;

        new_value
    }

    fn addhl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (new_value, is_overflow) = hl.overflowing_add(value);

        self.registers.f.substract = false;
        self.registers.f.carry = is_overflow;
        self.registers.f.half_carry = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;

        new_value
    }

    fn inc_16bit(&mut self, value: u16) -> u16 {
        let (new_value, _) = value.overflowing_add(1);

        new_value
    }

    fn dec_16bit(&mut self, value: u16) -> u16 {
        let (new_value, _) = value.overflowing_sub(1);

        new_value
    }

    fn swap(&mut self, value: u8) -> u8 {
        let upper = value & 0xF0;
        let lower = value & 0x0F;
        let new_value = (upper >> 4) + (lower << 4);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn cpl(&mut self, value: u8) -> u8 {
        let new_value = !value;

        self.registers.f.substract = true;
        self.registers.f.half_carry = true;

        new_value
    }

    fn ccf(&mut self) {
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    fn scf(&mut self) {
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    fn rlca(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_left(1);

        self.registers.f.zero = false; // diff
        self.registers.f.substract = false;
        self.registers.f.carry = (0x80 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rla(&mut self, value: u8) -> u8 {
        let new_value = (value << 1) + (self.registers.f.carry as u8);

        self.registers.f.zero = false; // diff
        self.registers.f.substract = false;
        self.registers.f.carry = (0x80 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rrca(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_right(1);

        self.registers.f.zero = false; // diff
        self.registers.f.substract = false;
        self.registers.f.carry = (0x1 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rra(&mut self, value: u8) -> u8 {
        let new_value = (value >> 1) + ((self.registers.f.carry as u8) << 7);

        self.registers.f.zero = false; // diff
        self.registers.f.substract = false;
        self.registers.f.carry = (0x1 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rlc(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_left(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x80 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rl(&mut self, value: u8) -> u8 {
        let new_value = (value << 1) + (self.registers.f.carry as u8);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x80 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rrc(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_right(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x1 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn rr(&mut self, value: u8) -> u8 {
        let new_value = (value >> 1) + ((self.registers.f.carry as u8) << 7);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x1 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn sla(&mut self, value: u8) -> u8 {
        let new_value = value << 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x80 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn sra(&mut self, value: u8) -> u8 {
        let new_value = (value >> 1) + (value & 0x80);

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x1 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn srl(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = (0x1 & value) != 0;
        self.registers.f.half_carry = false;

        new_value
    }

    fn bit(&mut self, b: u8, value: u8) {
        if b > 7u8 {
            panic!("BIT operation error: the test bit `b` should be between 0 and 7");
        }

        self.registers.f.zero = ((1u8 << b) & value) == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = true;
    }

    fn set(&mut self, b: u8, value: u8) -> u8 {
        let new_value = value | (1u8 << b);

        new_value
    }

    fn res(&mut self, b: u8, value: u8) -> u8 {
        let new_value = value & !(1u8 << b);

        new_value
    }
}
