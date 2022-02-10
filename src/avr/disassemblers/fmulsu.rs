use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::fmulsu::Fmulsu;

impl Disassembler for Fmulsu {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("FMULSU"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
