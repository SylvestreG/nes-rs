#![feature(unsized_locals)]

use std::fs;
use std::fs::File;
use std::io::Read;
use std::str;

use crate::cpu::memory::Memory;
use crate::cpu::imemory::IMemory;

pub struct Cartridge {
    mirroring: bool,
    battery_packed_ram: bool,
    trainer: bool,
    ignore_mirroring: bool,
    mapper: u8,
    pub prg_rom: Memory,
    pub prg_ram: Memory,
    pub chr_rom: Memory
}

impl IMemory for Cartridge {
    fn dump(&self) {
        self.prg_rom.dump()
    }

    fn dump_slice(&self, begin:usize, end:usize) {
        self.prg_rom.dump_slice(begin, end)
    }

    fn size(&self) -> u16 {
        0xa000
    }

    fn fetch(&self, address: u16) -> u8  {
        match address {
            0x0..=0x1FFF => {
                if self.prg_ram.size() > 0 {
                    return self.prg_ram.fetch(address);
                }

                panic!("invalid cartridge ram")
            }
            0x2000..=0x5FFF => return self.prg_rom.fetch(address - 0x2000),
            0x6000..=0x9FFF => {
                if self.prg_rom.size() <= 0x4000 {
                    return self.prg_rom.fetch(address - 0x6000);
                }
                return self.prg_rom.fetch(address - 0x2000);
            },
            _ => panic!("invalid cartridge rom")
        };
    }

    fn store(&mut self, _address: u16, _data: u8) {
        panic!("cannot write on ROM");
    }
}

impl Cartridge {
    pub fn new(file_name: &String) -> Self {
        println!("loading file : {}", file_name);
        let f = File::open(&file_name);
        let metadata = fs::metadata(&file_name);
        let mut buffer = vec![0; metadata.unwrap().len() as usize];
        f.unwrap().read(&mut buffer);

        let header = str::from_utf8(&buffer[0..3]);
        let size_prog_rom = buffer[4];
        let size_chr_rom = buffer[5];

        let mut mirroring = false;
        let mut battery = false;
        let mut trainer = false;
        let mut ignore_mirr = false;


        if buffer[6] & 0x1 == 0x1 { mirroring = true; }
        if buffer[6] & 0x2 == 0x2 { battery = true; }
        if buffer[6] & 0x4 == 0x4 { trainer = true; }
        if buffer[6] & 0x8 == 0x8 { ignore_mirr = true; }
        let mapper = buffer[6] >> 4;

        println!("{} nb_prog={} nb_chr={} f6={:#x} mirroring={} battery={} trainer={}, ignore={} mapper={}",
                 header.unwrap(), size_prog_rom, size_chr_rom, buffer[6], mirroring, battery, trainer,
                 ignore_mirr, mapper);

        if mapper != 0 {
            panic!("unsupported mapper");
        }

        let mut offset:usize = 16;
        if trainer { offset = offset + 512}

        let prog_rom_size = 16384 * (size_prog_rom as u16);
        println!("read from {:#8x} to {:#8x}", offset, offset + prog_rom_size as usize);
        let prog = &buffer[offset..offset + prog_rom_size as usize];

        offset = offset + prog_rom_size as usize;
        let chr_rom_size = 8192 * (size_chr_rom as u16);
        let chr = &buffer[offset..offset + chr_rom_size as usize];

        let mut ram = Memory::new(0);
        if mirroring {
            ram = Memory::new(0x2000);
        }


        Cartridge {
            mirroring,
            battery_packed_ram: battery,
            trainer,
            ignore_mirroring: ignore_mirr,
            mapper,
            prg_rom: Memory::from_data(prog),
            prg_ram: ram,
            chr_rom: Memory::from_data(chr)
        }
    }


}