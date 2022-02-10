pub trait Disassembler {
  fn disassemble(&self, address: u16) -> (String, Option<String>, Option<String>);
}
