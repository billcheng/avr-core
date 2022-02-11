use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::data::Data;

impl Disassembler for Data {
  fn disassemble(&self, _address: u32) -> (String, Option<String>, Option<String>) {
    (
      String::from(".DATA"),
      Some(format!("{}", self.opcode)),
      None,
    )
  }
}
