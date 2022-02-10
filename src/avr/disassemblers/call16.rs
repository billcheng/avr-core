use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::call16::Call16;

impl Disassembler for Call16 {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (String::from("CALL"), Some(format!("{}", self.k)), None)
  }
}
