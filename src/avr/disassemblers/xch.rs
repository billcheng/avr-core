use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::xch::Xch;

impl Disassembler for Xch {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("XCH"),
      Some(String::from("Z")),
      Some(format!("R{}", self.d)),
    )
  }
}
