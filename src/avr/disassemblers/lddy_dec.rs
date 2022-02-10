use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddy_dec::LddyDec;

impl Disassembler for LddyDec {
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
      Some(String::from("-Y")),
    )
  }
}
