
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

impl DiagnosticStatus {

    pub fn new() -> DiagnosticStatus {
        DiagnosticStatus{
            enable_instruction_trace : false,
            RFU_one : false,
            TLM_miss_location : false,
            TLB_shutdown: false,
            soft_reset_occurred: false,
            RFU_two : false,
            CP0_condition_bit: false,
        }
    }

    pub fn write(&mut self, data : u32){
        self.enable_instruction_trace = (data & 0x01000000) != 0;
        self.RFU_one = false;
        self.TLM_miss_location = (data & 0x00400000) != 0;
        self.TLB_shutdown = (data & 0x00200000) != 0;
        self.soft_reset_occurred = (data & 0x00100000) != 0;
        self.RFU_two = false;
        self.CP0_condition_bit =   (data & 0x00040000) != 0;
    }
}

#[derive(Debug)]
pub enum Mode{
    User,
    Supervisor,
    Kernel
}


impl Mode {

    pub fn write(&mut self, data: u32){
        // get two bits!
        let flag = data & (0x18);

        *self = match flag {
            0x0 => Mode::Kernel,
            0x1 => Mode::Supervisor,
            0x2 => Mode::User,
            _ => panic!("some weird bits in Mode::write /n")
        }
    }
}

#[derive(Default, Debug)]
pub struct InterruptMaskField {
    timer_interrupt : bool,
    external_interrupt_mask: [bool; 5],
    software_interrupts: [bool; 2]
}

impl InterruptMaskField {

    pub fn new() -> InterruptMaskField {
        InterruptMaskField{
            timer_interrupt : false,
            external_interrupt_mask: [false; 5],
            software_interrupts: [false; 2]
        }
    }

    pub fn write(&mut self, data : u32){
        self.timer_interrupt = (data & 0x00008000) != 0;
        self.external_interrupt_mask[0] = (data & 0x00004000) != 0;
        self.external_interrupt_mask[1] = (data & 0x00002000) != 0;
        self.external_interrupt_mask[2] = (data & 0x00001000) != 0;
        self.external_interrupt_mask[3] = (data & 0x00000800) != 0;
        self.external_interrupt_mask[4] = (data & 0x00000400) != 0;
        self.software_interrupts[0] = (data & 0x00000200) != 0;
        self.software_interrupts[1] = (data & 0x00000100) != 0;
    }
}

#[derive(Debug)]
pub struct StatusReg{
    //CU
    copro_use: [bool; 4],

    //RP
    low_power: bool,

    // FR
    additional_fp_regs: bool,

    // RE
    reverse_endian: bool,

    // DS
    diagnostic_status: DiagnosticStatus,

    // IM
    interrupt_mask: InterruptMaskField,

    // KX
    kernel_mode_enable_64_bit_addr : bool,

    // SX
    supervisor_mode_enable_64_bit_addr : bool,

    // UX
    user_mode_enable_64_bit_addr : bool,

    // KSU
    mode : Mode,

    // ERL
    error_level_enable : bool,

    // EXL
    exception_level : bool,

    // IE
    interrupt_enable : bool
}

impl StatusReg {

    pub fn new() -> StatusReg {
        StatusReg {
            copro_use: [false; 4],
            low_power: false,
            additional_fp_regs: false,
            reverse_endian: false,
            diagnostic_status: DiagnosticStatus::new(),
            interrupt_mask: InterruptMaskField::new(),
            kernel_mode_enable_64_bit_addr : false,
            supervisor_mode_enable_64_bit_addr : false,
            user_mode_enable_64_bit_addr : false,
            mode : Mode::Kernel,
            error_level_enable : false,
            exception_level : false,
            interrupt_enable : false
        }
    }

    pub fn write(&mut self, data : u32) {
        self.copro_use[0] = (data & 0x10000000) != 0;
        self.copro_use[1] = (data & 0x20000000) != 0;
        self.copro_use[2] = (data & 0x40000000) != 0;
        self.copro_use[3] = (data & 0x80000000) != 0;

        self.low_power              = (data & 0x08000000) != 0;
        self.additional_fp_regs     = (data & 0x04000000) != 0;
        self.reverse_endian         = (data & 0x02000000) != 0;

        self.diagnostic_status.write(data);
        self.interrupt_mask.write(data);

        self.kernel_mode_enable_64_bit_addr = (data & 0x80) != 0;
        self.supervisor_mode_enable_64_bit_addr = (data & 0x40) != 0;
        self.user_mode_enable_64_bit_addr = (data & 0x20) != 0;
        self.mode.write(data);
        self.error_level_enable = (data & 0x4) != 0;
        self.exception_level =  (data & 0x2) != 0;
        self.interrupt_enable = (data & 0x1) != 0;
    }
}