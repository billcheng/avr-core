use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::cbi::Cbi;

impl Disassembler for Cbi {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CBI"),
      Some(format!("{}", self.a)),
      Some(format!("{}", self.b)),
    )
  }
}
