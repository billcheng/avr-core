use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbrc::Sbrc;

impl Disassembler for Sbrc {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBRC"),
      Some(format!("R{}", self.r)),
      Some(format!("{}", self.b)),
    )
  }
}
