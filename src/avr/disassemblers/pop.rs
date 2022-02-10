use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::pop::Pop;

impl Disassembler for Pop {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("POP"), Some(format!("R{}", self.d)), None)
  }
}
