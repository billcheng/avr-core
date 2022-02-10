use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::sbiw::Sbiw;

impl Disassembler for Sbiw {
  fn disassemble(
    &self,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("SBIW"),
      format!("R{}:R{}", self.d + 1, self.d),
      format!("{}", self.k),
    )
  }
}
