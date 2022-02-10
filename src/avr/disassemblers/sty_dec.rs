use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sty_dec::StyDec;

impl Disassembler for StyDec {
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
      Some(String::from("-Y")),
      Some(format!("R{}", self.r)),
    )
  }
}
