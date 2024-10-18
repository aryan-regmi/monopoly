use std::{cell::RefCell, rc::Rc};

pub type RcCell<T> = Rc<RefCell<T>>;

pub(crate) const NUM_SPACES: usize = 40;
pub(crate) const NUM_CHANCE: usize = 16;
pub(crate) const NUM_COMMUNITY_CHEST: usize = 16;
