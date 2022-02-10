use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ror::Ror;

impl Disassembler for Ror {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("ROR"), Some(format!("R{}", self.d)), None)
  }
}
