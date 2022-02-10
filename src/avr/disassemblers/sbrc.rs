use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbrc::Sbrc;

impl Disassembler for Sbrc {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBRC"),
      format!("R{}", self.r),
      format!("{}", self.b),
    )
  }
}
