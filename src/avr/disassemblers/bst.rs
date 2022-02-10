use crate::avr::disassembler::Disassembler;
use crate::avr::instructions::bst::Bst;

impl Disassembler for Bst {
  fn disassemble(
    &self,
    _address: u16,
  ) -> (
    std::string::String,
    Option<std::string::String>,
    Option<std::string::String>,
  ) {
    (
      String::from("BST"),
      Some(format!("R{}", self.d)),
      Some(format!("{}", self.b)),
    )
  }
}
