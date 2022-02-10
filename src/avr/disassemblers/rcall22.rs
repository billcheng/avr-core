use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rcall22::Rcall22;

impl Disassembler for Rcall22 {
  fn disassemble(
    &self,
    address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("RCALL"),
      Some(format!("{}", (address as i64 + self.k as i64 + 1) as u32)),
      None,
    )
  }
}
