use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::in_io::In;

impl Disassembler for In {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("IN"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.a)),
    )
  }
}
