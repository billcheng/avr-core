use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::stx::Stx;

impl Disassembler for Stx {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(String::from("X")),
      Some(format!("R{}", self.r)),
    )
  }
}
