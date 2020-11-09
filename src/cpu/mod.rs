mod memory;
pub mod cartridge;
pub mod decode;
pub mod membus;
pub mod imemory;
pub mod execute;

use std::panic;

use crate::cpu::cartridge::Cartridge;
use crate::cpu::membus::Membus;
use crate::cpu::imemory::IMemory;
use crate::cpu::decode::{Decode, op_to_string};
use crate::cpu::execute::Execute;

pub struct Cpu {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8,
    flags: u8,

    mem_bus: Membus
}

impl Cpu {
    pub fn new(cartridge: Cartridge) -> Self {
        let membus = Membus::new(cartridge);
        let mut pc = membus.fetch(0xfffd) as u16;
        pc = pc << 8;
        pc = pc | membus.fetch(0xfffc) as u16;

        Cpu {
            pc: pc,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0x20,
            sp: 0xfd,
            flags: 0,

            mem_bus: membus
        }
    }

    pub fn tick(& mut self) {
        let opcode = Decode::new(self.mem_bus.fetch(self.pc));
        let to_read = opcode.bytes;
        let mut vec:Vec<u8> = Vec::new();

        for i in 1..to_read {
            let data = self.mem_bus.fetch(self.pc + to_read as u16);
            vec.push(data);
        }

        let op_name = op_to_string(opcode.op);

        let exe = Execute::new(opcode, vec);
        exe.execute(self);

        self.pc = self.pc + to_read as u16;
        self.dump(&op_name);
    }

    pub fn dump(&self, op_name:&String) {
        println!("cpu : pc={:#04x}({}) ac={:#02x} x={:#02x} y={:#02x} sr={:#02x} sp{:#02x} flags={:#02x}",
            self.pc, op_name, self.ac, self.x, self.y, self.sr, self.sp, self.flags)
    }
}