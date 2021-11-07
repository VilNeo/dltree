use crate::internal::leaf_impl::LeafImpl;
use crate::internal::node_impl::NodeImpl;
use crate::tree::Value;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum TreeElementImpl<IT, LT> {
    Node(Rc<RefCell<NodeImpl<IT, LT>>>),
    Leaf(Rc<RefCell<LeafImpl<IT, LT>>>),
}

impl<IT, LT> TreeElementImpl<IT, LT> {
    pub fn new(value: Value<IT, LT>, parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>) -> Self {
        match value {
            Value::Node(n) => Self::Node(Rc::new(RefCell::new(NodeImpl::new(n, parent)))),
            Value::Leaf(l) => Self::Leaf(Rc::new(RefCell::new(LeafImpl::new(l, parent)))),
        }
    }
}

impl<IT, LT> Clone for TreeElementImpl<IT, LT> {
    fn clone(&self) -> Self {
        match &self {
            TreeElementImpl::Node(n) => TreeElementImpl::Node(n.clone()),
            TreeElementImpl::Leaf(l) => TreeElementImpl::Leaf(l.clone()),
        }
    }
}
