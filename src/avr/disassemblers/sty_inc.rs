use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sty_inc::StyInc;

impl Disassembler for StyInc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(String::from("Y+")),
      Some(format!("R{}", self.r)),
    )
  }
}
