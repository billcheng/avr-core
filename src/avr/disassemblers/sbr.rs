use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbr::Sbr;

impl Disassembler for Sbr {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBR"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
