use crate::cpu::imemory::IMemory;
use crate::cpu::memory::Memory;
use crate::cpu::cartridge::Cartridge;

pub struct Membus {
    interal_ram: Memory,
    cartridge: Cartridge
}

impl IMemory for Membus {
    fn dump(&self) {
        self.interal_ram.dump()
    }

    fn dump_slice(&self, begin:usize, end:usize) {
    }

    fn size(&self) -> u16 {
        return 0xffff;
    }

    fn fetch(&self, address: u16) -> u8 {
        match address {
            0..=0x7FF => return self.interal_ram.fetch(address),
            0x800..=0xFFF => return self.interal_ram.fetch(address - 0x800),
            0x1000..=0x17FF => return self.interal_ram.fetch(address - 0x1000),
            0x1800..=0x1FFF => return self.interal_ram.fetch(address - 0x1800),

            0x6000..=0xFFFF => return self.cartridge.fetch(address - 0x6000),
            _ => panic!("Not addressed")
        }
    }

    fn store(&mut self, address: u16, data: u8) {
        match address {
            0..=0x7FF => self.interal_ram.store(address, data),
            0x800..=0xFFF => self.interal_ram.store(address - 0x800, data),
            0x1000..=0x17FF => self.interal_ram.store(address - 0x1000, data),
            0x1800..=0x1FFF => self.interal_ram.store(address - 0x1800, data),

            0x4020..=0xFFFF => self.cartridge.store(address - 0x4020, data),
            _ => panic!("Not addressed")
        }
    }
}

impl Membus {
    pub fn new(cartridge: Cartridge) -> Self {
        Membus {
            interal_ram: Memory::new(2048),
            cartridge
        }
    }
}