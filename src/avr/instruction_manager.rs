use crate::avr::operation::Operation;
use crate::avr::operations::adc::Adc;
use crate::avr::operations::add::Add;
use crate::avr::operations::adiw::Adiw;
use crate::avr::operations::and::And;
use crate::avr::operations::andi::Andi;
use crate::avr::operations::asr::Asr;
use crate::avr::operations::bclr::Bclr;
use crate::avr::operations::brbc::Brbc;
use crate::avr::operations::brbs::Brbs;
use crate::avr::operations::bset::Bset;

pub struct InstructionManager {}

impl InstructionManager {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, opcode: u16) -> Box<dyn Operation> {
    let is_adc = opcode & 0b1111_1100_0000_0000 == 0b0001_1100_0000_0000;
    if is_adc {
      return Box::new(Adc::new(opcode));
    }

    let is_add = opcode & 0b1111_1100_0000_0000 == 0b0000_1100_0000_0000;
    if is_add {
      return Box::new(Add::new(opcode));
    }

    let is_adiw = opcode & 0b1111_1111_0000_0000 == 0b1001_0110_0000_0000;
    if is_adiw {
      return Box::new(Adiw::new(opcode));
    }

    let is_and = opcode & 0b1111_1100_0000_0000 == 0b0010_0000_0000_0000;
    if is_and {
      return Box::new(And::new(opcode));
    }

    let is_andi = opcode & 0b1111_0000_0000_0000 == 0b0111_0000_0000_0000;
    if is_andi {
      return Box::new(Andi::new(opcode));
    }

    let is_asr = opcode & 0b1111_1110_0000_1111 == 0b1001_0100_0000_0101;
    if is_asr {
      return Box::new(Asr::new(opcode));
    }

    let is_bclr = opcode & 0b1111_1111_1000_1111 == 0b1001_0100_1000_1000;
    if is_bclr {
      return Box::new(Bclr::new(opcode));
    }

    let is_brbc = opcode & 0b1111_1100_0000_0000 == 0b1111_0100_0000_0000;
    if is_brbc {
      return Box::new(Brbc::new(opcode));
    }

    let is_brbs = opcode & 0b1111_1100_0000_0000 == 0b1111_0000_0000_0000;
    if is_brbs {
      return Box::new(Brbs::new(opcode));
    }

    let is_bset = opcode & 0b1111_1111_1000_1111 == 0b1001_0100_0000_1000;
    if is_bset {
      return Box::new(Bset::new(opcode));
    }

    panic!("Unknown opcode: 0x{:04x}", opcode);
  }
}
