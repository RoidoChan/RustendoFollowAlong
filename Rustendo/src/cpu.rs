use crate::interconnect;
use crate::cp0;

const NUM_GPR : usize = 32;
const NUM_FPR : usize = 32;


#[derive(Debug)]
pub struct Cpu {
    gpr_regs : [u64; NUM_GPR],
    fpr_regs : [f64; NUM_FPR],

    reg_pc : u64,
    reg_hi : u64,
    reg_lo : u64,

    FCR0   : f32,
    FCR31  : f32,

    reg_LLbit : bool, // TODO enum

    copro0 : cp0::CP0,
    interconnect: interconnect::Interconnect,
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        Cpu {
            gpr_regs : [0; NUM_GPR],
            fpr_regs : [0.0; NUM_FPR],
        
            reg_pc : 0,
            reg_hi : 0,
            reg_lo : 0,
        
            FCR0   : 0.0,
            FCR31  : 0.0,
        
            reg_LLbit : false, // TODO enum
        
            copro0 : cp0::CP0::new(),
            interconnect: interconnect,
        }
    }

    pub fn power_on_reset(&mut self){
        self.copro0.power_on_reset();
        self.reg_pc = 0xffff_ffff_bfc0_0000; // TODO move to const
    }

    pub fn run(&mut self){
        /*TODO*/
        loop {
            let opword = self.read_word(self.reg_pc);
            self.decode_instruction(opword);
            self.reg_pc += 4;
        }
    }

    fn decode_instruction(&mut self, opword : u32){
        if opword == 0{
            println!("NOP");
            return
        } 

        let opcode = (opword >> 26) & 0x3F;
            match opcode {
                0x0  => self.special_instr(opword),
                0x1  => self.bgez_instr(opword),
                0x4  => self.branch_on_equal_instr(opword),
                0x5  => self.branch_not_equal_instr(opword),
                0x8  => self.add_imm_instr(opword),
                0x9  => self.add_imm_unsigned_instr(opword),
                0xC  => self.andi_instr(opword),
                0xD  => self.ori_instr(opword),
                0xF  => self.lui_instr(opword),
                0x10 => self.mtc0_instr(opword),
                0x14 => self.branch_on_equal_likely_instr(opword),
                0x15 => self.branch_on_not_equal_likely_instr(opword),
                0x23 => self.load_word_instr(opword),
                0x2B => self.store_word_instr(opword),
                _ => panic!("unhandled opcode! {:#x}", opword),
            }
    }

    fn special_instr(&mut self, opword: u32){
        println!("special instruction!");

        // 'SPECIAL' mips instructions use last 6 bits
        let opcode = opword & 0x3F;

        match opcode{
            0x0  => self.shift_left_logical_instr(opword),
            0x12 => self.mflo_instr(opword),
            0x19 => self.multiply_unsigned_instr(opword),
            0x2  => self.shift_right_logical_instr(opword),
            0x23 => self.sub_unsigned_instr(opword),
            0x8  => self.jump_register_instr(opword),
            0x25 => self.or_instr(opword),
            0x26 => self.xor_instr(opword),
            _   => panic!("unhandled instruction {:#x}", opword),
        }
    }

    fn shift_right_logical_instr(&mut self, opword: u32){
        println!("SRL instruction!");
        panic!("unimplemented!");
    }

    fn shift_left_logical_instr(&mut self, opword: u32){
        println!("SLL instruction");
        panic!("unimplemented!");
    }

    fn jump_register_instr(&mut self, opword: u32){
        println!("jump register instruction!");
        panic!("unimplemented!");
    }

    fn add_imm_instr(&mut self, opword: u32){
        println!("add imm instruction!");


        panic!("unimplemented!");
    }

    fn split_opword(&self, opword: u32) -> (u32, u8, u8) {
        let imm = (opword & 0xFFFF);
        let rt = (opword >> 16) & 0x1F;
        let rs = (opword >> 21) & 0x1F; 

        (imm, rt as u8, rs as u8)
    }

    fn add_imm_unsigned_instr(&mut self, opword: u32){
        println!("addiu {:#x}", self.reg_pc);
        let (imm, rt, rs) = self.split_opword(opword);

        let imm = (imm as i32) as u64;
        let contents = self.gpr_regs[rs as usize];
        self.gpr_regs[rt as usize] = contents + imm;
    }

    fn sub_unsigned_instr(&mut self, opword: u32){
        println!("subu instr!");
        panic!("unimplemented!");
    }

    fn load_immediate_instr(&mut self, opword: u32){
        println!("li instruction!");
        panic!("unimplemented!");
    }

    fn store_word_instr(&mut self, opword: u32){
        println!("sw {:#x}", self.reg_pc);
        let (imm, rt, base) = self.split_opword(opword);
        let imm = (imm as i32) as u64;
        let base = self.gpr_regs[base as usize];
        let virt_addr = imm.wrapping_add(base);
        self.write_word(virt_addr, self.gpr_regs[rt as usize] as u32);
    }

    fn lui_instr(&mut self, opword : u32){
        println!("lui {:#x}", self.reg_pc);
        let (imm, rt, _) = self.split_opword(opword);
        let imm_shift = ((imm << 16) as i32) as u64;
        self.gpr_regs[rt as usize] = imm_shift;
    }

    fn mtc0_instr(&mut self, opword : u32){
        println!("mtc0 {:#x}", self.reg_pc);
        let rt = (opword >> 16) & 0x1F;
        let rd = (opword >> 11) & 0x1F;
        let data = self.gpr_regs[rt as usize];
        self.copro0.write_reg(rd, data);
    }

    fn mflo_instr(&mut self, opword: u32){
        println!("mflo instr");
        panic!("unimplemented!");
    }

    fn or_instr(&mut self, opword: u32){
        println!("or instruction");
        panic!("unimplemented!");
    }

    fn xor_instr(&mut self, opword: u32){
        println!("xor instruction");
        panic!("unimplemented!");
    }

    fn multiply_unsigned_instr(&mut self, opword: u32){
        println!("MULTU instruction");
        panic!("unimplemented!");
    }

    fn ori_instr(&mut self, opword : u32){
        println!("ori {:#x}", self.reg_pc);
        let (imm, rt, rs) = self.split_opword(opword);
        let imm = imm as i32;
        let rs_data = self.gpr_regs[rs as usize];
        self.gpr_regs[rt as usize] = rs_data | imm as u64;
    }

    fn load_word_instr(&mut self, opword: u32){
        println!("lw {:#x}", self.reg_pc);
        let (imm, rt, base) = self.split_opword(opword);
        let imm = (imm as i32) as u64;
        let reg_contents = self.gpr_regs[base as usize];
        let virt_addr = imm.wrapping_add(reg_contents);
        
        let val_to_write = self.read_word(virt_addr as u64);
        self.gpr_regs[rt as usize] = val_to_write as u64;
    }

    fn andi_instr(&mut self, opword: u32){
        println!("andi {:#x}", self.reg_pc);
        let (imm, rt, rs) = self.split_opword(opword);
        let imm = imm as i32;
        let contents = self.gpr_regs[rs as usize];
        self.gpr_regs[rt as usize] = (imm as u64) & contents;
    }

    fn bgez_instr(&mut self, opword: u32){
        println!("bgez instr!");
        panic!("unimplemented!");
    }

    fn branch_on_equal_likely_instr(&mut self, opword: u32){
        println!("beql {:#x}", self.reg_pc);
        let (offset, rt, rs) = self.split_opword(opword);
        let offset_shift = ((offset << 16) as i32) >> 14;

        if self.gpr_regs[rt as usize] == self.gpr_regs[rs as usize] {
            // execute next instr then jump
            let opword = self.read_word(self.reg_pc + 4);
            self.decode_instruction(opword);
            self.reg_pc = (offset_shift + 0x4) as u64;
        } else{
            // discard instr in delay slot
            self.reg_pc += 4;
        }
    }

    fn branch_on_not_equal_likely_instr(&mut self, opword: u32){
        println!("beqnl {:#x}", self.reg_pc);
        let (offset, rt, rs) = self.split_opword(opword);
        let offset_shift = (((offset << 16) as i32) >> 14) as u64;

        if self.gpr_regs[rt as usize] != self.gpr_regs[rs as usize] {
            // execute next instr then jump
            let opword = self.read_word(self.reg_pc + 4);
            self.decode_instruction(opword);
            self.reg_pc = (self.reg_pc.wrapping_add(offset_shift + 0x4)) as u64;
        } else{
            // discard instr in delay slot
            self.reg_pc += 4;
        }
    }

    fn branch_not_equal_instr(&mut self, opword: u32){
        println!("bne instruction!");
        panic!("unimplemented!");
    }

    fn branch_on_equal_instr(&mut self, opword: u32){
        println!("beq instruction!");
        panic!("unimplemented!");
    }

    fn read_word(&self, addr: u64) -> u32 {
            let phys = self.virtual_to_physical(addr);
            self.interconnect.read_word(phys as u32)
    }

    fn write_word(&mut self, addr: u64, data: u32){
        let phys = self.virtual_to_physical(addr);
        self.interconnect.write_word(phys as u32, data);
    }

    fn virtual_to_physical(&self, virtual_addr: u64) -> u64{
        // see table 5-3 in the data sheet
        let addr_bit_values = (virtual_addr >> 29) & 0b111;

        if addr_bit_values == 0b101 {
                //kseg 1
                virtual_addr - 0xffff_ffff_a000_0000
        }else{
            panic!("unrecognized virtual addr {:#x} {:#x}", virtual_addr, self.reg_pc);
        }
    }
}