
use crate::statusRegister::StatusReg;

// EP is bits 24:27 (4 bits) of config reg
// sets data transfer pattern


#[derive(Debug)]
enum ConfigEp{
    D, 
    DxxDxx,
    RFU
}

#[derive(Debug)]
enum ConfigBe {
    LittleEndian,
    BigEndian
}

mod ConfigRegister{

    #[derive(Debug)]
    pub struct ConfigRegister{
        contents : u32
    }

    const EC_OFFSET : u8 = 28;
    const EP_OFFSET : u8 = 24;
    const BE_OFFSET : u8 = 15;
    const CU_OFFSET : u8 = 2;
    const KO_OFFSET : u8 = 0;

    impl ConfigRegister {

        pub fn new() -> ConfigRegister{
            ConfigRegister{
                contents : 0b0_000_0000_00000110_0_1100_1000_110_0_000
        }
        }

        pub fn power_on_reset(&mut self) {
            // ep is bits 24:27, 0'ed
            self.contents = self.contents & (0b0000 << EP_OFFSET);
            // be is bit 15
            self.contents = self.contents & (0b0 << BE_OFFSET);
        }

        pub fn write(&mut self, data : u32){
            println!("written to status reg: {:#b}", data);
            self.contents = data;
        }
    }
}


#[derive(Debug)]
pub struct CP0 {
    reg_config : ConfigRegister::ConfigRegister,
    status_reg : StatusReg
}

impl CP0 {
    pub fn new() -> CP0 {
        CP0{
            reg_config : ConfigRegister::ConfigRegister::new(),
            status_reg : StatusReg::new() 
        }
    }

    pub fn power_on_reset(&mut self){
            // ts, sr, rp bits cleared to 0
            // ts - 21, sr - 20, rp -  27
            
            // 3:0 bits of config reg cleared to 0
            self.reg_config.power_on_reset();
    }

    pub fn write_reg(&mut self, index: u32, data: u64){
        match index {
            12 => {
                    self.status_reg.write(data as u32);
                    //println!("status register {:#?}\n", self.status_reg);
                    //panic!("done");
                 },
            16 => {
                self.reg_config.write(data as u32);
            },
            _  => panic!("trying to write to reg {} \n", index)
        }
    }
}
