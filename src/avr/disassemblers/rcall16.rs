use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rcall16::Rcall16;

impl Disassembler for Rcall16 {
  fn disassemble(
    &self,
    address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("RCALL"),
      format!("{}", (address as i32 + self.k as + i32 + 1) as u16),
      None,
    )
  }
}
