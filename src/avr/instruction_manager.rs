use crate::avr::operations::adiw::Adiw;
use crate::avr::operation::Operation;
use crate::avr::operations::adc::Adc;
use crate::avr::operations::add::Add;

pub struct InstructionManager {}

impl InstructionManager {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, opcode: u16) -> Box<dyn Operation> {
    return match opcode & 0b1111_1100_0000_0000 >> 8 {
      0b0001_1100 | 0b0001_1101 | 0b0001_1110 | 0b0001_1111 => Box::new(Adc::new(opcode)),
      0b0000_1100 | 0b0000_1101 | 0b0000_1110 | 0b0000_1111 => Box::new(Add::new(opcode)),
      0b1001_0110 => Box::new(Adiw::new(opcode)),
      _ => panic!("Unknown opcode: 0x{:04x}", opcode),
    };
  }
}
