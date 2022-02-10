use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::fmul::Fmul;

impl Disassembler for Fmul {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("FMUL"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
