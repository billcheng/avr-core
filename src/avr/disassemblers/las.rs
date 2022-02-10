use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::las::Las;

impl Disassembler for Las {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LAS"),
      Some(String::from("Z")),
      Some(format!("R{}", self.d)),
    )
  }
}
