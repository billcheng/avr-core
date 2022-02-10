use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbc::Sbc;

impl Disassembler for Sbc {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBC"),
      format!("R{}", self.d),
      format!("R{}", self.r),
    )
  }
}
