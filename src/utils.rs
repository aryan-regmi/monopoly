use std::{cell::RefCell, rc::Rc};

pub(crate) type RcCell<T> = Rc<RefCell<T>>;
