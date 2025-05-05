use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub type SharedRefCell<T> = Rc<RefCell<T>>;
pub fn shared_ref_cell<T>(data: T) -> SharedRefCell<T> {
    Rc::new(RefCell::new(data))
}

pub type SharedCell<T> = Rc<Cell<T>>;
pub fn shared_cell<T>(data: T) -> SharedCell<T> {
    Rc::new(Cell::new(data))
}
