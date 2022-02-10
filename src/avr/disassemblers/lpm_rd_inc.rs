use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lpm_rd_inc::LpmRdInc;

impl Disassembler for LpmRdInc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LPM"),
      Some(format!("R{}", self.d)),
      Some(String::from("Z+")),
    )
  }
}
