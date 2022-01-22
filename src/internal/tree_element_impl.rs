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
    pub(crate) fn update_parent(&mut self, parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>) {
        match self {
            TreeElementImpl::Node(n) => n.borrow_mut().parent = parent,
            TreeElementImpl::Leaf(l) => l.borrow_mut().parent = parent,
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

impl<IT: Clone, LT: Clone> crate::DeepClone for TreeElementImpl<IT, LT> {
    fn deep_clone(&self) -> Self {
        match &self {
            TreeElementImpl::Node(n) => {
                let new_node = Rc::new(RefCell::new(n.borrow().deep_clone()));
                new_node
                    .borrow_mut()
                    .children
                    .iter_mut()
                    .for_each(|c| c.update_parent(Some(Rc::downgrade(&new_node))));
                TreeElementImpl::Node(new_node)
            }
            TreeElementImpl::Leaf(l) => {
                TreeElementImpl::Leaf(Rc::new(RefCell::new(l.borrow().clone())))
            }
        }
    }
}
