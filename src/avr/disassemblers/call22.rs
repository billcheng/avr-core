use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::call22::Call22;

impl Disassembler for Call22 {
  fn disassemble(
    &self,
    address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CALL"),
      Some(format!("{}", (address as i64 + self.k as i64 + 1) as u32)),
      None,
    )
  }
}
