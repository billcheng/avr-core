use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::wdr::Wdr;

impl Disassembler for Wdr {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("WDR"),
      None,
      None,
    )
  }
}
