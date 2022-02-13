use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::brbs::Brbs;

impl Disassembler for Brbs {
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
        0 => "BRCS",
        1 => "BREQ",
        2 => "BRMI",
        3 => "BRVS",
        4 => "BRLT",
        5 => "BRHS",
        6 => "BRTS",
        7 => "BRIE",
        _ => panic!("Invalid bit number for BRBS {}", self.s),
      }),
      Some(format!("{}", (address as i64 + self.k as i64 + 1) as u32)),
      None
    )
  }
}
