use std::{cell::RefCell, rc::Rc};

pub(crate) type Ref<T> = Rc<RefCell<T>>;
