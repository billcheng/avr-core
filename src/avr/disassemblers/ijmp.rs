use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ijmp::Ijmp;

impl Disassembler for Ijmp {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("IJMP"), None, None)
  }
}
