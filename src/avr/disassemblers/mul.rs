use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::mul::Mul;

impl Disassembler for Mul {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("MUL"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
