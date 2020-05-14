
#[derive(Default, Debug)]
pub struct DiagnosticStatus {
    // ITS
    enable_instruction_trace : bool,

    //RFU
    RFU_one : bool,

    // BEV
    TLM_miss_location : bool,

    // TS
    TLB_shutdown: bool,

    // SR
    soft_reset_occurred: bool,

    RFU_two : bool,

    // CH
    CP0_condition_bit: bool,

    // CE, DE not used!
}

// status reg offsets
pub const CU : u8 = 28;
pub const RP : u8 = 27;
pub const FR : u8 = 26;
pub const RE : u8 = 25;
pub const DS : u8 = 16;
pub const IM : u8 = 8;
pub const KX : u8 = 7;
pub const SX : u8 = 6;
pub const UX : u8 = 5;
pub const KSU : u8 = 3;
pub const ERL : u8 = 2;
pub const EXL : u8 = 1;
pub const IE  : u8 = 0;

//diagnostic field offsets
pub const ITS : u8 = 24;
pub const BEV : u8 = 22;
pub const TS : u8 = 21;
pub const SR : u8 = 20;
pub const CH : u8 = 18;
pub const CE : u8 = 17;
pub const DE : u8 = 16;

#[derive(Debug)]
pub struct StatusReg{
    contents : u32,
}

impl StatusReg {

    pub fn new() -> StatusReg {
        StatusReg {
            contents : 0
        }
    }

    pub fn write(&mut self, data : u32) {
        println!("written to status reg: {:#b}", data);
        self.contents = data;
    }
}