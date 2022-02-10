use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lpm::Lpm;

impl Disassembler for Lpm {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LPM"),
      Some(String::from("R0")),
      Some(String::from("Z")),
    )
  }
}
