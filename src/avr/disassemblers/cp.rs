use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::cp::Cp;

impl Disassembler for Cp {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CP"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
