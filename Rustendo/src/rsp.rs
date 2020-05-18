use byteorder::{ByteOrder, BigEndian};

#[derive(Debug)]
pub struct Rsp{
    halt : bool,
    interrupt_enable : bool,
    broke : bool,

    imem : Box<[u8]>,
    dmem : Box<[u8]>
}

impl Rsp {

   pub fn new() -> Rsp {
       Rsp{
            halt : true,
            interrupt_enable : false,
            broke : false,
            imem : vec![0; 0x1000].into_boxed_slice(),
            dmem : vec![0; 0x1000].into_boxed_slice()
       }
   }

   pub fn read_status_reg(&self) -> u32 {
        (if self.halt { 0x1 } else { 0x0 }) |
        (if self.interrupt_enable { 0x1 } else { 0x0 } << 1) | 
        (if self.broke { 0x1 } else { 0x0 } << 2)
   }

   pub fn write_status_reg(&mut self, value: u32) {
        println!("value {:#b}", value);
        
        if (value & 0x1) == 1{
            self.halt = false;
        }

        if (value & 0x2) == 1{
            self.halt = true;
        }

        if (value & 0x4) == 1 {
            self.interrupt_enable = false;
        }

        if (value & 0x8) == 1 {
            self.interrupt_enable = true;
        }
   }

   pub fn write_imem(&mut self, addr: u32, value: u32){
        let loc_addr = addr - 0x400_1000;
        println!("{:#x}", loc_addr);
        BigEndian::write_u32(&mut self.imem[loc_addr as usize..], value);
   }

   pub fn read_imem(&self, addr : u32) -> u32 {
        let loc_addr = addr - 0x400_1000;
        BigEndian::read_u32(&self.imem[loc_addr as usize..])
   }

    pub fn write_dmem(&mut self, addr: u32, value: u32){
        let loc_addr = addr - 0x400_0000;
        BigEndian::write_u32(&mut self.dmem[loc_addr as usize..], value);
    }

    pub fn read_dmem(&self, addr : u32) -> u32 {
        let loc_addr = addr - 0x400_0000;
        BigEndian::read_u32(&self.dmem[loc_addr as usize..])
    }
}
