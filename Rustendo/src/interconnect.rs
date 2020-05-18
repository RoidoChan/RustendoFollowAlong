use byteorder::{ByteOrder, BigEndian};
use crate::mem_map::*;
use crate::rsp;
use crate::pif;
use crate::video_interface;

const RAM_SIZE: usize = 4 * 1024 * 1024;
const PIF_SIZE: usize = 2 * 1024;

#[derive(Debug)]
pub struct Interconnect {
    ram: Box<[u8]>,
    Rsp: rsp::Rsp,

    Pif: pif::Pif,

    VI : video_interface::video_interface
}

impl Interconnect {
    pub fn new(pif_rom: Box<[u8]>) -> Interconnect {
        Interconnect {
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            Rsp : rsp::Rsp::new(),
            Pif : pif::Pif::new(pif_rom),
            VI : video_interface::video_interface::new(),
        }
    }

    pub fn init(&mut self){
        self.Pif.init();
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;

        let result = match addr {
            0..=0x001F_FFFF => {
                println!("RD RAM MEM rng 1");
                0
            },

            PIF_ROM_START..=PIF_ROM_END => {
                let rel_addr = addr - PIF_ROM_START;
                self.Pif.read_pif_rom(rel_addr as u32)
            },

            PIF_RAM_START..=PIF_RAM_END => {
                let rel_addr = addr - PIF_RAM_START;
                println!("PIF RAM read {:#x}", rel_addr);
                let val = self.Pif.read_pif_ram(rel_addr as u32);
                println!("PIF RAM val {:#x}", val);
                val
            },

            SP_STATUS_REGISTER_START..=SP_STATUS_REGISTER_END => {
                println!("SP status register read {:#x}", addr);
                // on power up, base address contains 1
                let val = self.Rsp.read_status_reg();
                println!("val from sp status {:#b}", val);
                val
            },

            SP_DMA_BUSY => {
                //TODO
                println!("DMA busy read");
                0
            }

            PI_STATUS_REG_START..=PI_STATUS_REG_END => {
                println!("PI STATUS REG READ {:#x}", addr);
                self.Pif.read_status_reg()
            }

            VI_REG_START..=VI_REG_END => {
                println!("VI read {:#x}", addr);
                self.VI.read_vi_interface_reg()
            }

            AI_REG_START..=AI_REG_END => {
                println!("AI read {:#x}", addr);
                0
            },

            SI_REG_START..=SI_REG_END => {
                println!("SI read {:#x}", addr);
                0
            },

            SP_IMEM_START..=SP_IMEM_END => {
                //println!("SP IMEM read {:#x}", addr);
                self.Rsp.read_imem(addr as u32)
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
            SP_STATUS_REGISTER_START..=SP_STATUS_REGISTER_END => {
                println!("SP status register write!");
                self.Rsp.write_status_reg(value);
            },

            PI_STATUS_REG_START..=PI_STATUS_REG_END => {
                println!("PIF REG WRITE {:#x}", addr);
                self.Pif.write_status_reg(value);
            },

            PIF_RAM_START..=PIF_RAM_END => {
                println!("PIF Ram write");
                let rel_addr = addr - PIF_RAM_START;
                self.Pif.write_pif_ram(rel_addr as u32, value);
            },

            VI_REG_START..=VI_REG_END => {
                println!("VI write {:#x}", addr);
                self.VI.write_vi_interface_reg(value);
            },

            AI_REG_START..=AI_REG_END => {
                println!("AI write {:#x}", addr);
               // self.VI.write_vi_interface_reg(value);
            },

            SI_REG_START..=SI_REG_END => {
                println!("SI write {:#x}", addr);
                panic!();
            },

            SP_IMEM_START..=SP_IMEM_END => {
                println!("SP IMEM write {:#x}", addr);
                self.Rsp.write_imem(addr as u32, value);
            },

            _=> panic!("bad physical address {:#x}", addr)
        }
    }
}

