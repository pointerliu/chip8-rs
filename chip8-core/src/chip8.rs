use std::{fs::{self, File}, io::{self, Read}};

use log::{info, warn};

use crate::{consts::*, error::Chip8Error, inst::Instruction};

pub struct Chip8 {
    mem: Vec<u8>,
    regs: [u16; REG_SIZE],
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],

    cls_scn: bool
}

impl Chip8 {
    pub fn new(mut rom: File) -> Result<Self, io::Error> {
        let mut mem = vec![];
        rom.read_to_end(&mut mem)?;

        Ok(Chip8 {
            mem: mem,
            regs: [0; REG_SIZE],
            dt: 0,
            st: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            cls_scn: false
        })
    }

    fn write_reg(&mut self, reg_num: u8, value: u16) {
        if reg_num == 0xF {
            warn!("writing to vF flags register...")
        }
        self.regs[reg_num as usize] = value;
    }

    fn read_reg(&mut self, reg_num: u8) -> u16 {
        self.regs[reg_num as usize]
    }

    fn buzzer(&self) -> bool {
        self.st != 0
    }

    fn run_pipeline() {

    }

    fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        let buf = &self.mem[pc..pc + 2];
        ((buf[0] as u16) << 8) | (buf[1] as u16)
    }

    fn decode(inst: u16) -> Result<Instruction, Chip8Error> {
        inst.try_into()
    }

    pub fn get_scn_state(&mut self) -> bool {
        let f = self.cls_scn;
        self.cls_scn = false;
        f 
    }

    fn execute(&mut self, inst: Instruction) {
        let mut npc = self.pc + 2;
        match inst {
            Instruction::I0NNN(_) => { info!("meet instruction 0nnn => do nothing.."); },
            Instruction::I00E0 => { self.cls_scn = true; },
            Instruction::I00EE => {
                npc = self.stack[self.st as usize];
                self.st -= 1;
            },
            Instruction::I1NNN(addr) => {
                npc = addr;
            },
            Instruction::I2NNN(addr) => {
                self.st += 1;
                self.stack[self.st as usize] = self.pc;
                npc = addr;
            },
            Instruction::I3XKK(x, kk) => {
                if self.read_reg(x) == kk as u16 {
                    npc += 2;
                }
            },
            Instruction::I4XKK(x, kk) => {
                if self.read_reg(x) != kk as u16 {
                    npc += 2;
                }
            },
            Instruction::I5XY0(x, y) => {
                if self.read_reg(x) == self.read_reg(y) {
                    npc += 2;
                }
            },
            Instruction::I6XKK(x, kk) => self.write_reg(x, kk as u16),
            Instruction::I7XKK(x, kk) => {
                let tmp = self.read_reg(x) + kk as u16;
                self.write_reg(x, tmp);
            },
            Instruction::I8XY0(x, y) => {
                let tmp = self.read_reg(y);
                self.write_reg(x, tmp);
            },
            Instruction::I8XY1(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.write_reg(x, vx | vy);
            },
            Instruction::I8XY2(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.write_reg(x, vx & vy);
            },
            Instruction::I8XY3(_, _) => todo!(),
            Instruction::I8XY4(_, _) => todo!(),
            Instruction::I8XY5(_, _) => todo!(),
            Instruction::I8XY6(_, _) => todo!(),
            Instruction::I8XY7(_, _) => todo!(),
            Instruction::I8XYE(_, _) => todo!(),
            Instruction::I9XY0(_, _) => todo!(),
            Instruction::IANNN(_) => todo!(),
            Instruction::IBNNN(_) => todo!(),
            Instruction::ICXKK(_, _) => todo!(),
            Instruction::IDXYN(_, _, _) => todo!(),
            Instruction::IEX9E(_) => todo!(),
            Instruction::IEXA1(_) => todo!(),
            Instruction::IFX07(_) => todo!(),
            Instruction::IFX0A(_) => todo!(),
            Instruction::IFX15(_) => todo!(),
            Instruction::IFX18(_) => todo!(),
            Instruction::IFX1E(_) => todo!(),
            Instruction::IFX29(_) => todo!(),
            Instruction::IFX33(_) => todo!(),
            Instruction::IFX55(_) => todo!(),
            Instruction::IFX65(_) => todo!(),
        }
    }
}