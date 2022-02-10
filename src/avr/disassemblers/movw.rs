use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::movw::Movw;

impl Disassembler for Movw {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("MOVW"),
      Some(format!("R{}:R{}", self.d + 1, self.d)),
      Some(format!("R{}:R{}", self.r + 1, self.r)),
    )
  }
}
