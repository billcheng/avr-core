use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::nop::Nop;

impl Disassembler for Nop {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("NOP"), None, None)
  }
}
