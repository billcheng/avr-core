use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::brbs::Brbs;

impl Disassembler for Brbs {
  fn disassemble(
    &self,
    address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BRBS"),
      Some(format!("{}", self.s)),
      Some(format!("{}", (address as i64 + self.k as i64 + 1) as u32)),
    )
  }
}
