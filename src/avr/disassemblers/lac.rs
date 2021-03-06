use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lac::Lac;

impl Disassembler for Lac {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LAC"),
      Some(String::from("Z")),
      Some(format!("R{}", self.d)),
    )
  }
}
