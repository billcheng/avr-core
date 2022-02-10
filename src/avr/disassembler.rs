pub trait Disassembler {
  fn disassemble(&self) -> (String, Option<String>, Option<String>);
}
