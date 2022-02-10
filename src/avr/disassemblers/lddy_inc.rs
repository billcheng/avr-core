use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddy_inc::LddyInc;

impl Disassembler for LddyInc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LD"),
      Some(format!("R{}", self.d)),
      Some(String::from("Y+")),
    )
  }
}
