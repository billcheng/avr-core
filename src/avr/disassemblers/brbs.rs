use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::brbs::Brbs;

impl Disassembler for Brbs {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BRBS"),
      Some(format!("{}", self.s)),
      Some(format!("{}", self.k)),
    )
  }
}
