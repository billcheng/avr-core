use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::eijmp::Eijmp;

impl Disassembler for Eijmp {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("EIJMP"), None, None)
  }
}
