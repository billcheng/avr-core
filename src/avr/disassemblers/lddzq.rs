use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddzq::Lddzq;

impl Disassembler for Lddzq {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LD"),
      Some(format!("R{}", self.d)),
      Some(format!("Z+{}", self.q)),
    )
  }
}
