use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::styq::Styq;

impl Disassembler for Styq {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("ST"),
      Some(match self.q {
        0 => String::from("Y"),
        _ => format!("Y+{}", self.q),
      }),
      Some(format!("R{}", self.r)),
    )
  }
}
