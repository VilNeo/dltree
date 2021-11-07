use super::tree_element_impl::TreeElementImpl;
use crate::internal::TreeElementTrait;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
pub struct NodeImpl<IT, LT> {
    pub parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>,
    pub value: IT,
    pub children: Vec<TreeElementImpl<IT, LT>>,
}

impl<IT, LT> NodeImpl<IT, LT> {
    pub fn new(value: IT, parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>) -> Self {
        NodeImpl {
            parent,
            value,
            children: vec![],
        }
    }
}

impl<IT, LT> TreeElementTrait<IT, LT> for NodeImpl<IT, LT> {
    fn parent(&mut self) -> &mut Option<Weak<RefCell<NodeImpl<IT, LT>>>> {
        &mut self.parent
    }
}
