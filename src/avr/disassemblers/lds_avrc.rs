use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::lds_avrc::LdsAvrc;

impl Disassembler for LdsAvrc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("LDS"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.k)),
    )
  }
}
