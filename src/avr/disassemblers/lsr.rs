use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lsr::Lsr;

impl Disassembler for Lsr {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("LSR"), Some(format!("R{}", self.d)), None)
  }
}
