use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rjmp::Rjmp;

impl Disassembler for Rjmp {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("RJMP"), format!("{}", self.k), None)
  }
}
