use std::cell::RefCell;
use std::rc::Rc;

pub type IoPtr = Rc<RefCell<Io>>;

pub struct Io {}

impl Io {
  pub fn new() -> Self {
    Self {}
  }
}
