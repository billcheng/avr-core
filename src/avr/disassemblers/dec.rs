use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::dec::Dec;

impl Disassembler for Dec {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("DEC"), Some(format!("R{}", self.d)), None)
  }
}
