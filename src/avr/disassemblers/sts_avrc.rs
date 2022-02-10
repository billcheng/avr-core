use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sts_avrc::StsAvrc;

impl Disassembler for StsAvrc {
  fn disassemble(
    &self,
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
