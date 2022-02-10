use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::stzq::Stzq;

impl Disassembler for Stzq {
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
        _ => format!("Z+{}", self.q),
      }),
      Some(format!("R{}", self.r)),
    )
  }
}
