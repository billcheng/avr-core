use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::stz_dec::StzDec;

impl Disassembler for StzDec {
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
      Some(String::from("-Z")),
      Some(format!("R{}", self.r)),
    )
  }
}
