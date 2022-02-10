use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbc::Sbc;

impl Disassembler for Sbc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBC"),
      Some(format!("R{}", self.d)),
      Some(format!("R{}", self.r)),
    )
  }
}
