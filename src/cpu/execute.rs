use crate::cpu::decode::Decode;
use crate::cpu::decode::Opcode;
use crate::cpu::decode::AddressMode;
use crate::cpu::decode::op_to_string;
use crate::cpu::Cpu;
use crate::cpu::imemory::IMemory;

pub struct Execute {
    opcode: Decode,
    data: Vec<u8>
}

impl Execute {
    pub fn new(opcode: Decode, data: Vec<u8>) -> Self {
        Execute {
            opcode,
            data
        }
    }

    pub fn execute(&self, cpu: & mut Cpu) {
        let get_imm = || -> u8 { self.data[0] };
        let get_zpg = || -> u16 { self.data[0] as u16};
        let get_zpg_y = || -> u16 { self.data[0] as u16 + cpu.y as u16};
        let get_zpg_x = || -> u16 { self.data[0] as u16 + cpu.x as u16};
        let get_abs = || -> u16 { self.data[0] as u16 | (self.data[1] as u16) << 8};
        let get_abs_y = || -> u16 { (self.data[0] as u16 | (self.data[1] as u16) << 8) + cpu.y as u16};
        let get_abs_x = || -> u16 { (self.data[0] as u16 | (self.data[1] as u16) << 8) + cpu.x as u16};

        match self.opcode.op {
            Opcode::SEI => cpu.flags = cpu.flags | 0x4,
            Opcode::CLD => cpu.flags = cpu.flags & !0x8,
            Opcode::LDX => {
                match self.opcode.mode {
                    AddressMode::Imm => cpu.x = get_imm(),
                    AddressMode::Zpg => cpu.x = cpu.mem_bus.fetch(get_zpg()),
                    AddressMode::ZpgY => cpu.x = cpu.mem_bus.fetch(get_zpg_y()),
                    AddressMode::Abs => cpu.x = cpu.mem_bus.fetch(get_abs()),
                    AddressMode::AbsY => cpu.x = cpu.mem_bus.fetch(get_abs_y()),
                    _ => panic!("invalid address mode for LDX")
                }
            },
            Opcode::INC => {
                match self.opcode.mode {
                    AddressMode::Zpg => cpu.mem_bus.store(get_zpg(), cpu.mem_bus.fetch(get_zpg()) + 1),
                    AddressMode::ZpgX => cpu.mem_bus.store(get_zpg_x(), cpu.mem_bus.fetch(get_zpg_x()) + 1),
                    AddressMode::Abs => cpu.mem_bus.store(get_abs(), cpu.mem_bus.fetch(get_abs()) + 1),
                    AddressMode::AbsX => cpu.mem_bus.store(get_abs_x(), cpu.mem_bus.fetch(get_abs_x()) + 1),
                    _ => panic!("invalid address mode for INC")
                }
            },
            Opcode::TXS => {},
            Opcode::LDA => {
                match self.opcode.mode {
                    AddressMode::Imm => cpu.ac = get_imm(),
                    AddressMode::Zpg => cpu.ac = cpu.mem_bus.fetch(get_zpg()),
                    AddressMode::ZpgX => cpu.ac = cpu.mem_bus.fetch(get_zpg_x()),
                    AddressMode::Abs => cpu.x = cpu.mem_bus.fetch(get_abs()),
                    AddressMode::AbsX => cpu.x = cpu.mem_bus.fetch(get_abs_x()),
                    AddressMode::AbsY => cpu.x = cpu.mem_bus.fetch(get_abs_y()),
                    _ => panic!("invalid address mode for LDX")
                }
            }
            _ => panic!("opcode not implemented = {}", op_to_string(self.opcode.op))
        }

        true;
    }
}