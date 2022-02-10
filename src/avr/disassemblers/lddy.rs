use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddy::Lddy;

impl Disassembler for Lddy {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LD"),
      Some(format!("R{}", self.d)),
      Some(String::from("Y")),
    )
  }
}
