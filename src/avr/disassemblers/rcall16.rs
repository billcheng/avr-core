use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rcall16::Rcall16;

impl Disassembler for Rcall16 {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("RCALL"),
      format!("{}", self.k),
      None,
    )
  }
}
