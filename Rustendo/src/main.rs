mod cpu;
mod cp0;
mod n64;
mod interconnect;
mod statusRegister;
mod mem_map;
mod rsp;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;



fn main() {
    let mut args = env::args();
    //let rom_file_name = args.nth(1).unwrap();
    let pif_file_name = args.nth(1).unwrap();    

    let pif_rom = load_binary(pif_file_name);
    //let bin_rom = load_binary(rom_file_name);

    let mut n64 = n64::N64::new(pif_rom);
    n64.power_on_reset();
    n64.run();
}

fn load_binary<P: AsRef<Path>>(path : P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf);
    file_buf
}