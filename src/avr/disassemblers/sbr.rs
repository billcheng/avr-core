use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbr::Sbr;

impl Disassembler for Sbr {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBR"),
      format!("R{}", self.d),
      format!("{}", self.k),
    )
  }
}
