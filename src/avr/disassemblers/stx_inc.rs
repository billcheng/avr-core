use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::stx_inc::StxInc;

impl Disassembler for StxInc {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(String::from("X+")),
      Some(format!("R{}", self.r)),
    )
  }
}
