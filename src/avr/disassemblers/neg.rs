use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::neg::Neg;

impl Disassembler for Neg {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("NEG"), Some(format!("R{}", self.d)), None)
  }
}
