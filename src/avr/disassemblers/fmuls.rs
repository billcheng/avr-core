use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::fmuls::Fmuls;

impl Disassembler for Fmuls {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("FMULS"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
