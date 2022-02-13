use crate::avr::core::Core;
use crate::avr::core_type::CoreType;

mod avr;

fn main() {
    let mut core = Core::new(CoreType::Bits16, 8192, 8192);
    core.load_code(&vec![0b0001_1100_0000_0000]);
    let (x, y, z) = core.disassemble(0);
    println!("{:4x} {:?} {:?} {:?}", 0, x, y, z);
    // core.single_step();
    // println!("{:?}", core.registers);
}