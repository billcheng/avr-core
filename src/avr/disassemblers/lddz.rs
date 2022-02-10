use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddz::Lddz;

impl Disassembler for Lddz {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LD"),
      Some(format!("R{}", self.d)),
      Some(String::from("Z")),
    )
  }
}
