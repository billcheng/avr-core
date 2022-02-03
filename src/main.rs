use crate::avr::core::Core;

mod avr;

fn main() {
    let mut core = Core::new(8192, 8192);
    core.load_code(&vec![0b0001_1100_0000_0000]);
    core.single_step();
    // println!("{:?}", core.registers);
}
