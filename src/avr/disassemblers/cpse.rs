use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::cpse::Cpse;

impl Disassembler for Cpse {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CPSE"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.r)),
    )
  }
}
