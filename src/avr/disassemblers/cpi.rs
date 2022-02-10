use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::cpi::Cpi;

impl Disassembler for Cpi {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CPI"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
