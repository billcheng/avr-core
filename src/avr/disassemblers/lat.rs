use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lat::Lat;

impl Disassembler for Lat {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LAT"),
      Some(String::from("Z")),
      Some(format!("R{}", self.d)),
    )
  }
}
