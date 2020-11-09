use crate::cpu::imemory::IMemory;

pub struct Memory {
    size: u16,
    data: Vec<u8>,
}

impl IMemory for Memory {
    fn dump(&self) {
        hexdump::hexdump(&self.data.as_slice())
    }

    fn dump_slice(&self, begin:usize, end:usize) {
        hexdump::hexdump(&self.data[begin..end])
    }

    fn size(&self) -> u16 {
        return self.size;
    }

    fn fetch(&self, address: u16) -> u8 {
        return self.data[address as usize];
    }

    fn store(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }
}

impl Memory {
    pub fn new(size: u16) -> Memory {
        Memory {
            size: size,
            data: vec![0; size as usize],
        }
    }

    pub fn from_data(data: &[u8]) -> Memory {
        Memory {
            size: data.len() as u16,
            data: data.to_vec()
        }
    }
}