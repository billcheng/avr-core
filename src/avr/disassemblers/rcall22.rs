use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rcall22::Rcall22;

impl Disassembler for Rcall22 {
  fn disassemble(
    &self,
    address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("RCALL"), format!("{}", (address as i32 + self.k as + i32 + 1) as u16),, None)
  }
}
