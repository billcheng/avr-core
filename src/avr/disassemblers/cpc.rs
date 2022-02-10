use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::cpc::Cpc;

impl Disassembler for Cpc {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("CPC"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
