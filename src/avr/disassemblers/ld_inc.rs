use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ld_inc::LdInc;

impl Disassembler for LdInc {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LD"),
      Some(format!("R{}", self.d)),
      Some(String::from("X+")),
    )
  }
}
