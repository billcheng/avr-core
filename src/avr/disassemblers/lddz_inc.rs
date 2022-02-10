use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddz_inc::LddzInc;

impl Disassembler for LddzInc {
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
      Some(String::from("Z+")),
    )
  }
}
