use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::icall22::Icall22;

impl Disassembler for Icall22 {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("ICALL"), None, None)
  }
}
