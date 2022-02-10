use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::swap::Swap;

impl Disassembler for Swap {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(format!("R{}", self.d)),
      None,
    )
  }
}
