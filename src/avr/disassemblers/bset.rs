use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::bset::Bset;

impl Disassembler for Bset {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BSET"),
      Some(format!(
        "{}",
        match self.s {
          0 => "C",
          1 => "Z",
          2 => "N",
          3 => "V",
          4 => "S",
          5 => "H",
          6 => "T",
          7 => "I",
          _ => panic!("Invalid bit number for BSET {}", self.s),
        }
      )),
      None,
    )
  }
}
