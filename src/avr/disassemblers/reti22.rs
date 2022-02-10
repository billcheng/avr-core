use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::reti22::Reti22;

impl Disassembler for Reti22 {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("RETI"), None, None)
  }
}
