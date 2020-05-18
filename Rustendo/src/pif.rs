use byteorder::{ByteOrder, BigEndian};

#[derive(Debug)]
pub struct Pif{
    status_reg : u32,
    pif_rom: Box<[u8]>,
    pif_ram: Box<[u8]>
}

impl Pif {
    pub fn new(pif_rom: Box<[u8]>) -> Pif {
        Pif{
            status_reg : 0,
            pif_rom: pif_rom, 
            pif_ram: vec![0; 0x1000].into_boxed_slice()
        }
    }

    pub fn init(&mut self){
        println!("init");
        self.pif_ram[0x3C] = 0x8;
    }

    pub fn read_status_reg(&self) -> u32 {
        self.status_reg
    }

    pub fn write_status_reg(&mut self, data: u32) {
        println!("write {} to pif status", data);
        self.status_reg = data;
    }

    pub fn read_pif_rom(&self, addr: u32) -> u32{
        BigEndian::read_u32(&self.pif_rom[addr as usize..])
    }

    pub fn read_pif_ram(&self, addr: u32) -> u32{
        BigEndian::read_u32(&self.pif_ram[addr as usize..])
    }

    pub fn write_pif_ram(&mut self, addr: u32, value: u32){
        BigEndian::write_u32(&mut self.pif_ram[addr as usize..], value);
    }
}