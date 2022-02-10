use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::mulsu::Mulsu;

impl Disassembler for Mulsu {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("MULSU"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
