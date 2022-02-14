use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::bclr::Bclr;

impl Disassembler for Bclr {
  fn disassemble(
    &self,
    _address: u32,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      format!(
        "CL{}",
        match self.s {
          0 => "C",
          1 => "Z",
          2 => "N",
          3 => "V",
          4 => "S",
          5 => "H",
          6 => "T",
          7 => "I",
          _ => panic!("Invalid bit number for BCLR {}", self.s),
        }
      ),
      None,
      None,
    )
  }
}
