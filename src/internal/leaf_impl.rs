use crate::internal::node_impl::NodeImpl;
use crate::internal::TreeElementTrait;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
pub struct LeafImpl<IT, LT> {
    pub parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>,
    pub value: LT,
}

impl<IT, LT> LeafImpl<IT, LT> {
    pub fn new(value: LT, parent: Option<Weak<RefCell<NodeImpl<IT, LT>>>>) -> Self {
        LeafImpl { parent, value }
    }
}

impl<IT, LT> TreeElementTrait<IT, LT> for LeafImpl<IT, LT> {
    fn parent(&mut self) -> &mut Option<Weak<RefCell<NodeImpl<IT, LT>>>> {
        &mut self.parent
    }
}
