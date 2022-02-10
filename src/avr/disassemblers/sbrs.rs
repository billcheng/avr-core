use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbrs::Sbrs;

impl Disassembler for Sbrs {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBRS"),
      Some(format!("R{}", self.r)),
      Some(format!("{}", self.b)),
    )
  }
}
