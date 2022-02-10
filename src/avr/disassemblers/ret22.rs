use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::ret22::Ret22;

impl Disassembler for Ret22 {
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
