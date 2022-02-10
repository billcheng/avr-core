use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::muls::Muls;

impl Disassembler for Muls {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("MULS"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
