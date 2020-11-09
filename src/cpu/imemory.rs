pub trait IMemory {
    fn dump(&self);
    fn dump_slice(&self, begin:usize, end:usize);
    fn size(&self) -> u16;
    fn fetch(&self, address: u16) -> u8 ;
    fn store(&mut self, address: u16, data: u8);
}