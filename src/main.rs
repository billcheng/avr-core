use crate::avr::avr_type::AvrType;
use crate::avr::core::Core;
use crate::avr::core_type::CoreType;

mod avr;

fn main() {
    let mut core = Core::new(CoreType::Bits16, AvrType::Avre, 8192, 8192);
    // core.load_code(&vec![0x9503, 0xD001, 0xcffd, 0x9513, 0x9508, 0, 0, 0, 0, 0]);
    core.load_code(&vec![0xef8f, 0xe09a, 0x96cf, 0xcffc, 0x00]);
    for _ in 0..4 {
        let pc = core.get_program_counter();
        let (x, y, z) = core.disassemble(pc);
        println!("{:4x} {:?} {:?} {:?}", pc, x, y, z);
        core.single_step();
    }
}
