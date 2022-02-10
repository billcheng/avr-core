use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::call22::Call22;

impl Disassembler for Call22 {
  fn disassemble(
    &self,
    address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CALL"),
      Some(format!("{}", (address as i32 + self.k as i32 + 1) as u16)),
      None,
    )
  }
}
