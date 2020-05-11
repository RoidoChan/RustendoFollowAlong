
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

#[derive(Debug)]
pub struct RegConfig{
    reg_config_ep: ConfigEp,
    some_val : u8,
    reg_config_be: ConfigBe,
    some_val_two : [bool; 11],
    CU : bool,
    K0_cacheUsed : bool 
}


impl RegConfig {

    pub fn new() -> RegConfig{
       RegConfig{
        reg_config_ep : ConfigEp::D,
        some_val : 0x6,
        reg_config_be : ConfigBe::BigEndian,
        some_val_two : [true, true, false, false, true, false, false, false, true, true, false],
        CU: false,
        K0_cacheUsed : false
       }
    }

    pub fn power_on_reset(&mut self) {
        self.reg_config_ep = ConfigEp::D;
        self.reg_config_be = ConfigBe::BigEndian;
    }

    pub fn write(&mut self, data : u32){
        println!("value passed is {:#034b}", data);
    }
}


#[derive(Debug)]
pub struct CP0 {
    reg_config : RegConfig,
    status_reg : StatusReg
}

impl CP0 {
    pub fn new() -> CP0 {
        CP0{
            reg_config : RegConfig::new(),
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
