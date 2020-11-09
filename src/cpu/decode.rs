use std::fmt;

#[derive(Copy, Clone)]
pub enum Opcode {
    BRK,
    BPL,
    JSR,
    BMI,
    RTI,
    BVC,
    RTS,
    BVS,
    BCC,
    LDY,
    BCS,
    CPY,
    BNE,
    CPX,
    BEQ,
    BIT,
    STY,
    ORA,
    AND,
    EOR,
    ADC,
    STA,
    LDA,
    CMP,
    SBC,
    ASL,
    ROL,
    LSR,
    ROR,
    STX,
    LDX,
    DEC,
    INC,
    PHP,
    CLC,
    PLP,
    SEC,
    PHA,
    CLI,
    PLA,
    SEI,
    DEY,
    CLV,
    TAY,
    TYA,
    JMP,
    INY,
    CLD,
    INX,
    SED,
    TXA,
    TAX,
    TXS,
    NOP,
    DEX,
    TSX,
}

pub fn op_to_string(opcode: Opcode) -> String {
    let mut str = String::new();
    match opcode {
        Opcode::BRK => str = str + "BRK",
        Opcode::BPL => str = str + "BPL",
        Opcode::JSR => str = str + "JSR",
        Opcode::BMI => str = str + "BMI",
        Opcode::RTI => str = str + "RTI",
        Opcode::BVC => str = str + "BVC",
        Opcode::RTS => str = str + "RTS",
        Opcode::BVS => str = str + "BVS",
        Opcode::BCC => str = str + "BCC",
        Opcode::LDY => str = str + "LDY",
        Opcode::BCS => str = str + "BCS",
        Opcode::CPY => str = str + "CPY",
        Opcode::BNE => str = str + "BNE",
        Opcode::CPX => str = str + "CPX",
        Opcode::BEQ => str = str + "BEQ",
        Opcode::BIT => str = str + "BIT",
        Opcode::STY => str = str + "STY",
        Opcode::ORA => str = str + "ORA",
        Opcode::AND => str = str + "AND",
        Opcode::EOR => str = str + "EOR",
        Opcode::ADC => str = str + "ADC",
        Opcode::STA => str = str + "STA",
        Opcode::LDA => str = str + "LDA",
        Opcode::CMP => str = str + "CMP",
        Opcode::SBC => str = str + "SBC",
        Opcode::ASL => str = str + "ASL",
        Opcode::ROL => str = str + "ROL",
        Opcode::LSR => str = str + "LSR",
        Opcode::ROR => str = str + "ROR",
        Opcode::STX => str = str + "STX",
        Opcode::LDX => str = str + "LDX",
        Opcode::DEC => str = str + "DEC",
        Opcode::INC => str = str + "INC",
        Opcode::PHP => str = str + "PHP",
        Opcode::CLC => str = str + "CLC",
        Opcode::PLP => str = str + "PLP",
        Opcode::SEC => str = str + "SEC",
        Opcode::PHA => str = str + "PHA",
        Opcode::CLI => str = str + "CLI",
        Opcode::PLA => str = str + "PLA",
        Opcode::SEI => str = str + "SEI",
        Opcode::DEY => str = str + "DEY",
        Opcode::CLV => str = str + "CLV",
        Opcode::TAY => str = str + "TAY",
        Opcode::TYA => str = str + "TYA",
        Opcode::JMP => str = str + "JMP",
        Opcode::INY => str = str + "INY",
        Opcode::CLD => str = str + "CLD",
        Opcode::INX => str = str + "INX",
        Opcode::SED => str = str + "SED",
        Opcode::TXA => str = str + "TXA",
        Opcode::TAX => str = str + "TAX",
        Opcode::TXS => str = str + "TXS",
        Opcode::NOP => str = str + "NOP",
        Opcode::DEX => str = str + "DEX",
        Opcode::TSX => str = str + "TSX"
    }

    return str;
}

#[derive(Clone, Copy)]
pub enum AddressMode {
    Acc,
    Abs,
    AbsX,
    AbsY,
    Imm,
    Ind,
    XInd,
    IndY,
    Rel,
    Zpg,
    ZpgX,
    ZpgY,
    Impl,
}

pub struct Decode {
    raw_op: u8,
    pub op: Opcode,
    pub mode: AddressMode,
    cycles: u8,
    pub bytes: u8,
    boundary_hint: bool,
    page_hint: bool,
}

impl Decode {
    fn update_val(&mut self, op: Opcode, mode: AddressMode, cycles: u8, bytes: u8, boudary: bool, page: bool) {
        self.op = op;
        self.mode = mode;
        self.cycles = cycles;
        self.bytes = bytes;
        self.boundary_hint = boudary;
        self.page_hint = page
    }

    fn b0_stage_mux(&mut self, b: u8, c: u8) {
        match b {
            1 => match c {
                1 => self.update_val(Opcode::BIT, AddressMode::Zpg, 3, 2, false, false),
                3 => self.update_val(Opcode::BIT, AddressMode::Abs, 4, 3, false, false),
                _ => panic!("unknown BIT opcode={}", self.raw_op)
            }
            4 => match c {
                1 => self.update_val(Opcode::STY, AddressMode::Zpg, 3, 2, false, false),
                3 => self.update_val(Opcode::STY, AddressMode::Abs, 4, 3, false, false),
                5 => self.update_val(Opcode::STY, AddressMode::ZpgX, 4, 2, false, false),
                _ => panic!("unknown STY opcode={}", self.raw_op)
            }
            5 => match c {
                0 => self.update_val(Opcode::LDY, AddressMode::Imm, 2, 2, false, false),
                1 => self.update_val(Opcode::LDY, AddressMode::Zpg, 3, 2, false, false),
                3 => self.update_val(Opcode::LDY, AddressMode::Abs, 4, 3, false, false),
                5 => self.update_val(Opcode::LDY, AddressMode::ZpgX, 4, 3, false, false),
                7 => self.update_val(Opcode::LDY, AddressMode::AbsX, 4, 3, true, false),
                _ => panic!("unknown LDY opcode={}", self.raw_op)
            }
            6 => match c {
                0 => self.update_val(Opcode::CPY, AddressMode::Imm, 2, 2, false, false),
                1 => self.update_val(Opcode::CPY, AddressMode::Zpg, 3, 2, false, false),
                2 => self.update_val(Opcode::CPY, AddressMode::Abs, 4, 3, false, false),
                _ => panic!("unknown CPY opcode={}", self.raw_op)
            }
            7 => match c {
                0 => self.update_val(Opcode::CPX, AddressMode::Imm, 2, 2, false, false),
                1 => self.update_val(Opcode::CPX, AddressMode::Zpg, 3, 2, false, false),
                2 => self.update_val(Opcode::CPX, AddressMode::Abs, 4, 3, false, false),
                _ => panic!("unknown CPX opcode={}", self.raw_op)
            }
            _ => panic!("unknown b0 opcode={}", self.raw_op)
        }
    }

    fn b1_stage_mux(&mut self, b: u8, c: u8) {
        let addr_mode = vec![AddressMode::XInd, AddressMode::Zpg, AddressMode::Imm, AddressMode::Abs, AddressMode::IndY, AddressMode::ZpgX, AddressMode::AbsY, AddressMode::AbsX];
        if self.raw_op == 24 {
            panic!("unknown STA opcode={}", self.raw_op)
        }

        let byte_pattern = vec![2, 2, 2, 3, 2, 2, 3, 3];
        let cycle_pattern = vec![6, 3, 2, 4, 5, 4, 4, 4];
        let boundary_pattern = vec![false, false, false, false, true, false, true, true];

        match b {
            0 => {
                self.update_val(Opcode::ORA, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            1 => {
                self.update_val(Opcode::AND, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            2 => {
                self.update_val(Opcode::EOR, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            3 => {
                self.update_val(Opcode::ADC, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            4 => {
                let bytes = vec![2, 2, 0, 3, 2, 2, 3, 3];
                let cycles_mode = vec![6, 3, 0, 4, 6, 4, 5, 5];
                self.update_val(Opcode::STA, addr_mode[c as usize], cycles_mode[c as usize], bytes[c as usize], false, false)
            }
            5 => {
                self.update_val(Opcode::LDA, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            6 => {
                self.update_val(Opcode::CMP, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            7 => {
                self.update_val(Opcode::SBC, addr_mode[c as usize], cycle_pattern[c as usize], byte_pattern[c as usize], boundary_pattern[c as usize], false)
            }
            _ => panic!("unknown b1 opcode={}", self.raw_op)
        }
    }

    fn b2_stage_mux(&mut self, b: u8, c: u8) {
        let addr_mode = vec![AddressMode::Imm, AddressMode::Zpg, AddressMode::Acc, AddressMode::Abs, AddressMode::Imm, AddressMode::ZpgX, AddressMode::Imm, AddressMode::AbsX];
        let addr_mode_y = vec![AddressMode::Imm, AddressMode::Zpg, AddressMode::Acc, AddressMode::Abs, AddressMode::Imm, AddressMode::ZpgY, AddressMode::Imm, AddressMode::AbsY];

        let byte_pattern1 = vec![0, 2, 1, 3, 0, 2, 0, 3];
        let byte_pattern2 = vec![0, 2, 0, 3, 0, 2, 0, 3];
        let cycle_pattern1 = vec![0, 5, 2, 6, 0, 6, 0, 7];
        let cycle_pattern2 = vec![0, 5, 0, 6, 0, 6, 0, 7];

        match b {
            0 => match c {
                0 => panic!("unknown ASL opcode={}", self.raw_op),
                4 => panic!("unknown ASL opcode={}", self.raw_op),
                6 => panic!("unknown ASL opcode={}", self.raw_op),
                _ => self.update_val(Opcode::ASL, addr_mode[c as usize], cycle_pattern1[c as usize], byte_pattern1[c as usize], false, false)
            },
            1 => match c {
                0 => panic!("unknown ROL opcode={}", self.raw_op),
                4 => panic!("unknown ROL opcode={}", self.raw_op),
                6 => panic!("unknown ROL opcode={}", self.raw_op),
                _ => self.update_val(Opcode::ROL, addr_mode[c as usize], cycle_pattern1[c as usize], byte_pattern1[c as usize], false, false)
            }
            2 => match c {
                0 => panic!("unknown LSR opcode={}", self.raw_op),
                4 => panic!("unknown LSR opcode={}", self.raw_op),
                6 => panic!("unknown LSR opcode={}", self.raw_op),
                _ => self.update_val(Opcode::LSR, addr_mode[c as usize], cycle_pattern1[c as usize], byte_pattern1[c as usize], false, false)
            }
            3 => match c {
                0 => panic!("unknown ROR opcode={}", self.raw_op),
                4 => panic!("unknown ROR opcode={}", self.raw_op),
                6 => panic!("unknown ROR opcode={}", self.raw_op),
                _ => self.update_val(Opcode::ROR, addr_mode[c as usize], cycle_pattern1[c as usize], byte_pattern1[c as usize], false, false)
            }
            4 => match c {
                0 => panic!("unknown STX opcode={}", self.raw_op),
                2 => panic!("unknown STX opcode={}", self.raw_op),
                4 => panic!("unknown STX opcode={}", self.raw_op),
                6 => panic!("unknown STX opcode={}", self.raw_op),
                7 => panic!("unknown STX opcode={}", self.raw_op),
                _ => {
                    let bytes = vec![0, 2, 0, 3, 0, 2, 0, 0];
                    let cycles_mode = vec![0, 3, 0, 4, 0, 4, 0, 0];
                    self.update_val(Opcode::STX, addr_mode_y[c as usize], cycles_mode[c as usize], bytes[c as usize], false, false)
                }
            }
            5 => match c {
                2 => panic!("unknown LDX opcode={}", self.raw_op),
                4 => panic!("unknown LDX opcode={}", self.raw_op),
                6 => panic!("unknown LDX opcode={}", self.raw_op),
                _ => {
                    let bytes = vec![2, 2, 0, 3, 0, 2, 0, 3];
                    let cycles_mode = vec![2, 3, 0, 4, 0, 4, 0, 4];
                    let boundary = vec![false, false, false, false, false, false, false, true];
                    self.update_val(Opcode::LDX, addr_mode_y[c as usize], cycles_mode[c as usize], bytes[c as usize], boundary[c as usize], false)
                }
            }
            6 => match c {
                0 => panic!("unknown DEC opcode={}", self.raw_op),
                2 => panic!("unknown DEC opcode={}", self.raw_op),
                4 => panic!("unknown DEC opcode={}", self.raw_op),
                6 => panic!("unknown DEC opcode={}", self.raw_op),
                _ => self.update_val(Opcode::DEC, addr_mode[c as usize], cycle_pattern2[c as usize], byte_pattern2[c as usize], false, false)
            }
            7 => match c {
                0 => panic!("unknown INC opcode={}", self.raw_op),
                2 => panic!("unknown INC opcode={}", self.raw_op),
                4 => panic!("unknown INC opcode={}", self.raw_op),
                6 => panic!("unknown INC opcode={}", self.raw_op),
                _ => self.update_val(Opcode::INC, addr_mode[c as usize], cycle_pattern2[c as usize], byte_pattern2[c as usize], false, false)
            }
            _ => panic!("unknown b2 opcode={}", self.raw_op)
        }
    }

    fn a_stage_mux(mut self, a: u8, b: u8, c: u8) -> Self {
        match self.raw_op {
            0x00 => self.update_val(Opcode::BRK, AddressMode::Impl, 7, 1, false, false),
            0x08 => self.update_val(Opcode::PHP, AddressMode::Impl, 3, 1, false, false),
            0x10 => self.update_val(Opcode::BPL, AddressMode::Rel, 2, 2, false, true),
            0x18 => self.update_val(Opcode::CLC, AddressMode::Impl, 2, 1, false, false),
            0x20 => self.update_val(Opcode::JSR, AddressMode::Abs, 6, 3, false, false),
            0x28 => self.update_val(Opcode::PLP, AddressMode::Impl, 4, 1, false, false),
            0x30 => self.update_val(Opcode::BMI, AddressMode::Rel, 2, 2, false, true),
            0x38 => self.update_val(Opcode::SEC, AddressMode::Impl, 2, 1, false, false),
            0x40 => self.update_val(Opcode::RTI, AddressMode::Impl, 6, 1, false, false),
            0x48 => self.update_val(Opcode::PHA, AddressMode::Impl, 3, 1, false, false),
            0x4C => self.update_val(Opcode::JMP, AddressMode::Abs, 3, 2, false, false),
            0x50 => self.update_val(Opcode::BVC, AddressMode::Rel, 2, 2, false, true),
            0x58 => self.update_val(Opcode::CLI, AddressMode::Impl, 2, 1, false, false),
            0x60 => self.update_val(Opcode::RTS, AddressMode::Impl, 6, 1, false, false),
            0x68 => self.update_val(Opcode::PLA, AddressMode::Impl, 4, 1, false, false),
            0x6C => self.update_val(Opcode::JMP, AddressMode::Ind, 5, 3, false, false),
            0x70 => self.update_val(Opcode::BVS, AddressMode::Rel, 2, 1, false, true),
            0x78 => self.update_val(Opcode::SEI, AddressMode::Impl, 2, 1, false, false),
            0x88 => self.update_val(Opcode::DEY, AddressMode::Impl, 2, 1, false, false),
            0x90 => self.update_val(Opcode::BCC, AddressMode::Rel, 2, 2, false, true),
            0x98 => self.update_val(Opcode::TYA, AddressMode::Impl, 2, 1, false, false),
            0xA8 => self.update_val(Opcode::TAY, AddressMode::Impl, 2, 1, false, false),
            0xB0 => self.update_val(Opcode::BCS, AddressMode::Rel, 2, 2, false, true),
            0xB8 => self.update_val(Opcode::CLV, AddressMode::Impl, 2, 1, false, false),
            0xC8 => self.update_val(Opcode::INY, AddressMode::Impl, 2, 1, false, false),
            0xD0 => self.update_val(Opcode::BNE, AddressMode::Rel, 2, 2, false, true),
            0xD8 => self.update_val(Opcode::CLD, AddressMode::Impl, 2, 1, false, false),
            0xE8 => self.update_val(Opcode::INX, AddressMode::Impl, 2, 1, false, false),
            0xF0 => self.update_val(Opcode::BEQ, AddressMode::Impl, 2, 2, false, true),
            0xF8 => self.update_val(Opcode::SED, AddressMode::Impl, 2, 1, false, false),

            0x8A => self.update_val(Opcode::TXA, AddressMode::Impl, 2, 1, false, false),
            0x9A => self.update_val(Opcode::TXS, AddressMode::Impl, 2, 1, false, false),
            0xAA => self.update_val(Opcode::TAX, AddressMode::Impl, 2, 1, false, false),
            0xBA => self.update_val(Opcode::TSX, AddressMode::Impl, 2, 1, false, false),
            0xCA => self.update_val(Opcode::DEX, AddressMode::Impl, 2, 1, false, false),
            0xEA => self.update_val(Opcode::NOP, AddressMode::Impl, 2, 1, false, false),

            _ => {
                match a {
                    0 => self.b0_stage_mux(b, c),
                    1 => self.b1_stage_mux(b, c),
                    _ => self.b2_stage_mux(b, c)
                }
            }
        }

        self
    }

    pub fn new(op: u8) -> Self {
        let a = op & 0x03;
        let b = (op & 0xe0) >> 5;
        let c = (op & 0x1C) >> 2;

        let ponay = Decode {
            raw_op: op,
            op: Opcode::ADC,
            mode: AddressMode::Imm,
            cycles: 4,
            bytes: 2,
            boundary_hint: false,
            page_hint: false,
        }.a_stage_mux(a, b, c);

        ponay
    }
}
