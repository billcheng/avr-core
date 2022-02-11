use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::brbc::Brbc;

impl Disassembler for Brbc {
  fn disassemble(
    &self,
    address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from(match self.s {
        0 => "BRCC",
        1 => "BRNE",
        2 => "BRPL",
        3 => "BRVC",
        4 => "BRGE",
        5 => "BRHC",
        6 => "BRTC",
        7 => "BRID",
        _ => unreachable!(),
      }),
      Some(format!("{}", self.s)),
      Some(format!("{}", (address as i64 + self.k as i64 + 1) as u32)),
    )
  }
}
