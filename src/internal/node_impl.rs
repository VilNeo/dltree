use super::tree_element_impl::TreeElementImpl;
use crate::internal::TreeElementTrait;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Weak;

#[derive(Debug)]
pub struct NodeImpl<IT, LT> {
    pub parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>,
    pub value: IT,
    pub children: VecDeque<TreeElementImpl<IT, LT>>,
}

impl<IT, LT> NodeImpl<IT, LT> {
    pub fn new(value: IT, parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>) -> Self {
        NodeImpl {
            parent,
            value,
            children: VecDeque::new(),
        }
    }
}

impl<IT, LT> TreeElementTrait<IT, LT> for NodeImpl<IT, LT> {
    fn parent(&mut self) -> &mut Option<Weak<RefCell<NodeImpl<IT, LT>>>> {
        &mut self.parent
    }
}

impl<IT: Clone, LT: Clone> crate::DeepClone for NodeImpl<IT, LT> {
    fn deep_clone(&self) -> Self {
        NodeImpl {
            parent: None,
            value: self.value.clone(),
            children: self.children.iter().map(|c| c.deep_clone()).collect(),
        }
    }
}
