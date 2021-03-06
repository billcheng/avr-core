use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::add::Add;

impl Disassembler for Add {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ADD"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
