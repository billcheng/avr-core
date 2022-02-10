use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sub::Sub;

impl Disassembler for Sub {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
