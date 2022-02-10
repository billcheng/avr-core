use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ret16::Ret16;

impl Disassembler for Ret16 {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("RET"), None, None)
  }
}
