use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::push::Push;

impl Disassembler for Push {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("PUSH"), Some(format!("R{}", self.d)), None)
  }
}
