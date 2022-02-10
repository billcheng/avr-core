use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::inc::Inc;

impl Disassembler for Inc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("IN"), Some(format!("R{}", self.d)), None)
  }
}