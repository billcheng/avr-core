pub trait ReadOnlyMemory {
  fn read(&self, address: u16) -> u16;
}
