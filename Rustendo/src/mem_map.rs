
pub const SP_DMEM_START : usize = 0x0400_1000;
pub const SP_DMEM_END : usize = 0x0400_1FFF;

pub const SP_IMEM_START : usize = 0x0400_1000;
pub const SP_IMEM_END : usize = 0x0400_1FFF;

pub const VI_REG_START: usize = 0x0440_0000;
pub const VI_REG_END: usize = 0x044F_FFFF; 

pub const AI_REG_START : usize = 0x450_0000;
pub const AI_REG_END : usize = 0x45F_FFFF;

pub const SI_REG_START : usize = 0x0480_0000;
pub const SI_REG_END : usize = 0x048F_FFFF;

pub const PIF_ROM_START: usize = 0x1FC0_0000;
pub const PIF_ROM_END: usize =  0x1FC0_07BF;

pub const PIF_RAM_START: usize = 0x1FC0_07C0;
pub const PIF_RAM_END: usize = 0x1FC0_07FF;

pub const SP_STATUS_REGISTER_START: usize = 0x404_0010;
pub const SP_STATUS_REGISTER_END:   usize = 0x404_0013;

pub const SP_DMA_BUSY: usize = 0x404_0018;

pub const PI_REG_START: usize = 0x460_0000;
pub const PI_STATUS_REG_START: usize = 0x460_0010;
pub const PI_STATUS_REG_END: usize = 0x460_0013;
pub const PI_REG_END: usize = 0x46F_FFFF;