use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::call16::Call16;

impl Disassembler for Call16 {
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
