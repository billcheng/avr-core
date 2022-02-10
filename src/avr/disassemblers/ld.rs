use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ld::Ld;

impl Disassembler for Ld {
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
      Some(String::from("X")),
    )
  }
}
