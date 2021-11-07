use crate::internal::node_impl::NodeImpl;
use std::cell::RefCell;
use std::rc::Weak;

pub mod leaf_impl;
pub mod node_impl;
pub mod tree_element_impl;

pub trait TreeElementTrait<IT, LT> {
    fn parent(&mut self) -> &mut Option<Weak<RefCell<NodeImpl<IT, LT>>>>;
}
