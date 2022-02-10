use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::icall16::Icall16;

impl Disassembler for Icall16 {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ICALL"),
      None,
      None,
    )
  }
}
