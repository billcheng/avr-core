use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sts::Sts;

impl Disassembler for Sts {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("STS"),
      Some(format!("{}", self.k)),
      Some(format!("R{}", self.r)),
    )
  }
}
