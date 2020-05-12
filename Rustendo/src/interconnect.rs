use byteorder::{ByteOrder, BigEndian};

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
        if addr < 0x001F_FFFF{
            println!("RD RAM MEM rng 1");
            0
        }else if addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0 {
            let rel_addr = addr - 0x1fc0_0000;
            BigEndian::read_u32(&self.pif_rom[rel_addr as usize..]) 
        }else if addr >= 0x1FC0_07C0 && addr < 0x1FC0_07FF{
                println!("PIF Joypad read");
                0
        }else if addr >= 0x400_0000 && addr < 0x400_0FFF {
            println!("SP DMEM read");
            0
        }else if addr >= 0x400_2000 && addr < 0x403_FFFF {
            println!("unused mem read");
            0
        }else if addr >= 0x404_0000 && addr < 0x404_1000 {
            println!("SP register read {:#x}", addr);
            // on power up, contains 1
            1
        }else if addr >= 0x0410_0000 && addr < 0x041F_FFFF{
                println!("DP command registers");
                0
        }else if addr >= 0x0480_0018 && addr < 0x0480_001B{
            println!("SI status reg read {:#x}", addr);
            0
        }else if addr >= 0x0600_0000 && addr < 0x07FF_FFFF{
                println!("N64 DD drive read");
                0
        }else{
                panic!("bad physical address {:#x}", addr)
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32){
        // pif boot rom range
        if addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0 {
            let rel_addr = addr - 0x1fc0_0000;
            BigEndian::write_u32(&mut self.ram[rel_addr as usize..], value)
        }else if addr >= 0x404_0000 && addr < 0x404_1000 {
            println!("SP register write {:#x}", addr);
        }else{
            panic!("bad physical address {:#x}", addr)
        }
    }
}

