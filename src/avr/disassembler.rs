pub trait Disassembler {
  fn disassemble(&self, address: u32) -> (String, Option<String>, Option<String>);
}
