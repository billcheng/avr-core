use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::stz_inc::StzInc;

impl Disassembler for StzInc {
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
      Some(String::from("Z+")),
      Some(format!("R{}", self.r)),
    )
  }
}
