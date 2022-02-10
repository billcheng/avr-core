use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::eicall::Eicall;

impl Disassembler for Eicall {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("EICALL"), None, None)
  }
}
