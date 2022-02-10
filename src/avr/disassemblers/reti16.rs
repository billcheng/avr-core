use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::reti16::Reti16;

impl Disassembler for Reti16 {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("RETI"), None, None)
  }
}
