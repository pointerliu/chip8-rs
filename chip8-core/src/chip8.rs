use std::{
    fs::File, io::{self, Read}
};

use log::{info, warn};
use rand::Rng;

use crate::{consts::*, error::Chip8Error, inst::Instruction};

pub struct Chip8 {
    mem: Vec<u8>,
    regs: [u8; REG_SIZE],
    reg_i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    keys: [bool; 16],

    cls_scn: bool,
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
            cls_scn: false,
            reg_i: 0,
            keys: [false; 16]
        })
    }

    fn write_reg(&mut self, reg_num: u8, value: u8) {
        if reg_num == 0xF {
            warn!("writing to vF flags register...")
        }
        self.regs[reg_num as usize] = value;
    }

    fn read_reg(&mut self, reg_num: u8) -> u8 {
        self.regs[reg_num as usize]
    }

    fn buzzer(&self) -> bool {
        self.st != 0
    }

    fn run_pipeline() {}

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
            Instruction::I0NNN(_) => {
                info!("meet instruction 0nnn => do nothing..");
            }
            Instruction::I00E0 => {
                self.cls_scn = true;
            }
            Instruction::I00EE => {
                npc = self.stack[self.st as usize];
                self.st -= 1;
            }
            Instruction::I1NNN(addr) => {
                npc = addr;
            }
            Instruction::I2NNN(addr) => {
                self.st += 1;
                self.stack[self.st as usize] = self.pc;
                npc = addr;
            }
            Instruction::I3XKK(x, kk) => {
                if self.read_reg(x) == kk as u8 {
                    npc += 2;
                }
            }
            Instruction::I4XKK(x, kk) => {
                if self.read_reg(x) != kk as u8 {
                    npc += 2;
                }
            }
            Instruction::I5XY0(x, y) => {
                if self.read_reg(x) == self.read_reg(y) {
                    npc += 2;
                }
            }
            Instruction::I6XKK(x, kk) => self.write_reg(x, kk as u8),
            Instruction::I7XKK(x, kk) => {
                let tmp = self.read_reg(x) + kk as u8;
                self.write_reg(x, tmp);
            }
            Instruction::I8XY0(x, y) => {
                let tmp = self.read_reg(y);
                self.write_reg(x, tmp);
            }
            Instruction::I8XY1(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.write_reg(x, vx | vy);
            }
            Instruction::I8XY2(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.write_reg(x, vx & vy);
            }
            Instruction::I8XY3(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.write_reg(x, vx ^ vy);
            }
            Instruction::I8XY4(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                let res: u8 = vx + vy;
                let c = ((res >> 8) != 0) as u8;
                self.write_reg(x, res);
                self.write_reg(0xF, c);
            }
            Instruction::I8XY5(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                let res: u8 = vx - vy;
                let c = (vx > vy) as u8;
                self.write_reg(x, res as u8);
                self.write_reg(0xF, c);
            }
            Instruction::I8XY6(x, y) => {
                let vx = self.read_reg(x);
                self.write_reg(0xF, ((vx & 1) == 1) as u8);
                self.write_reg(x, vx >> 1);
            }
            Instruction::I8XY7(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                let res: u8 = vy - vx;
                let c = (vy > vx) as u8;
                self.write_reg(x, res as u8);
                self.write_reg(0xF, c);
            }
            Instruction::I8XYE(x, y) => {
                let vx = self.read_reg(x);
                self.write_reg(0xF, ((vx & (1 << 7)) == 1) as u8);
                self.write_reg(x, vx << 1);
            }
            Instruction::I9XY0(x, y) => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                if vx != vy {
                    npc += 2
                }
            }
            Instruction::IANNN(nnn) => {
                self.reg_i = nnn;
            }
            Instruction::IBNNN(nnn) => {
                npc = nnn + self.read_reg(0) as u16
            }
            Instruction::ICXKK(x, kk) => {
                let mut rng = rand::thread_rng();
                let rd: u8 = rng.gen();
                self.write_reg(x, (rd & kk) as u8); 
            }
            Instruction::IDXYN(x, y, nibble) => {
               todo!() 
            }
            Instruction::IEX9E(x) => {
                let vx = self.read_reg(x);
                if self.keys[vx as usize] {
                    npc += 2
                }
            }
            Instruction::IEXA1(x) => {
                let vx = self.read_reg(x);
                if !self.keys[vx as usize] {
                    npc += 2
                }
            }
            Instruction::IFX07(x) => {
                self.write_reg(x, self.dt as u8);
            }
            Instruction::IFX0A(x) => {
                for k in 0..16 {
                    if self.keys[k] {
                        self.write_reg(x, k as u8);
                    }
                }
            }
            Instruction::IFX15(x) => {
                self.dt = self.read_reg(x) as u8
            }
            Instruction::IFX18(x) => {
                self.st = self.read_reg(x) as u8
            }
            Instruction::IFX1E(x) => {
                self.reg_i += self.read_reg(x) as u16
            }
            Instruction::IFX29(x) => {
                todo!()
            }
            Instruction::IFX33(x) => {
                todo!()
            }
            Instruction::IFX55(x) => {
                todo!()
            }
            Instruction::IFX65(x) => {
                todo!()
            }
        }
    }
}
