mod cpu;
mod opcodes;

use crate::nes::cpu::CPU;

#[allow(dead_code)]
pub struct NES {
    pub cpu: CPU,
    //pub ppu: PPU,
    //pub ram: RAM
    //pub apu: APU,
}

#[allow(dead_code)]
impl NES {
    pub fn new() -> Self {
        NES{
            cpu: CPU::new(),
        }
    }
}