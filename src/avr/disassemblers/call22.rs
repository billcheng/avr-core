use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::call22::Call22;

impl Disassembler for Call22 {
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
