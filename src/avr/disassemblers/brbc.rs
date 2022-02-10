use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::brbc::Brbc;

impl Disassembler for Brbc {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BRBC"),
      Some(format!("{}", self.s)),
      Some(format!("{}", self.k)),
    )
  }
}
