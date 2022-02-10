use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::jmp::Jmp;

impl Disassembler for Jmp {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("JMP"), Some(format!("{}", self.k)), None)
  }
}
