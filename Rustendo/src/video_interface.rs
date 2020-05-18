#[derive(Debug)]
pub struct video_interface{
    interrupt_half_line: u32
}

impl video_interface {

    pub fn new() -> video_interface{
        video_interface{
            interrupt_half_line : 0
        }
    }

    pub fn read_vi_interface_reg(&self) -> u32 {
        self.interrupt_half_line
    }

    pub fn write_vi_interface_reg(&mut self, data : u32) {
        println!("value {}", data);
        self.interrupt_half_line = data & 0x3FF;
    }
}