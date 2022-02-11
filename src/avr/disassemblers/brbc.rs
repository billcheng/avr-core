use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::brbc::Brbc;

impl Disassembler for Brbc {
  fn disassemble(
    &self,
    address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BRBC"),
      Some(format!("{}", self.s)),
      Some(format!("{}", (address as i64 + self.k as i64 + 1) as u32)),
    )
  }
}
