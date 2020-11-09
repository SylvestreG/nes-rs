use std::env;
use std::fs;
use std::rc::Rc;
use crate::cpu::cartridge::Cartridge;

mod cpu;

fn help() {
    println!("usage: nes-rs <string>
    the cartridge path.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let metadata = fs::metadata(&args[1]);

            if !metadata.is_ok() {
                panic!("file {} does not exist", args[1]);
            }
        },
        _ => {
            help();
        }
    }

    let cartridge = cpu::cartridge::Cartridge::new(&args[1]);
    let mut cpu = cpu::Cpu::new(cartridge);
    while true {
        cpu.tick();
    }
}