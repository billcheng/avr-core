use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::rjmp::Rjmp;

impl Disassembler for Rjmp {
  fn disassemble(
    &self,
    address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("RJMP"),
      Some(format!("{}", address as i64 + self.k as i64 + 1)),
      None,
    )
  }
}
