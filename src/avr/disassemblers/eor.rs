use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::eor::Eor;

impl Disassembler for Eor {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("EOR"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
