use byteorder::{ByteOrder, BigEndian};
use crate::mem_map::*;

const RAM_SIZE: usize = 4 * 1024 * 1024;
const PIF_SIZE: usize = 2 * 1024;

#[derive(Debug)]
pub struct Interconnect {
    pif_rom: Vec<u8>,
    ram: Vec<u8>
}

impl Interconnect {
    pub fn new(pif_rom: Vec<u8>) -> Interconnect {
        Interconnect {
            pif_rom: pif_rom, 
            ram: vec![0; RAM_SIZE]
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;

        let result = match addr {
            0..=0x001F_FFFF => {
                println!("RD RAM MEM rng 1");
                0
            },

            PIF_ROM_START...PIF_RAM_START => {
                let rel_addr = addr - PIF_ROM_START;
                BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
            },

            PIF_JOYPAD_START...PIF_JOYPAD_END => {
                println!("PIF Joypad read");
                0
            },

            SP_REGISTER_START...SP_REGISTER_END => {
                println!("SP register read {:#x}", addr);
                // on power up, contains 1
                1
            },

            PIF_REG_START...PIF_REG_END => {
                println!("PIF REG READ {:#x}", addr);
                1
            }

            VI_REG_START...VI_REG_END => {
                println!("VI read {:#x}", addr);
                1
            }

            _=> {
                panic!("bad physical address {:#x}", addr)
            }
        };

        result
    }

    pub fn write_word(&mut self, addr: u32, value: u32){
        // pif boot rom range
        let addr = addr as usize;

        match addr {
            PIF_ROM_START..=PIF_RAM_START => {
                let rel_addr = addr - PIF_ROM_START;
                BigEndian::write_u32(&mut self.ram[rel_addr as usize..], value)
            },

            SP_REGISTER_START..=SP_REGISTER_END => {
                println!("SP register write!");
            },

            PIF_REG_START...PIF_REG_END => {
                println!("PIF REG WRITE {:#x}", addr);
            },

            VI_REG_START...VI_REG_END => {
                println!("VI write {:#x}", addr);
            }

            _=> panic!("bad physical address {:#x}", addr)
        }
    }
}

