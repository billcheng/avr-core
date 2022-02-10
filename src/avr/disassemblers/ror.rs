use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ror::Ror;

impl Disassembler for Ror {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("ROR"), format!("R{}", self.d), None)
  }
}
