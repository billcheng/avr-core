use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lddyq::Lddyq;

impl Disassembler for Lddyq {
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
      Some(format!("Y+{}", self.q)),
    )
  }
}
