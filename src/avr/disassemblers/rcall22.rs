use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rcall22::Rcall22;

impl Disassembler for Rcall22 {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("RCALL"), format!("{}", self.k), None)
  }
}
