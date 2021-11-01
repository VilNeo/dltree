use super::node::Node;
use crate::internal::leaf_impl::LeafImpl;
use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree::Tree;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Leaf<IT, LT> {
    leaf: Rc<RefCell<LeafImpl<IT, LT>>>,
}

impl<IT, LT> Clone for Leaf<IT, LT> {
    fn clone(&self) -> Self {
        Leaf {
            leaf: self.leaf.clone(),
        }
    }
}

impl<IT, LT> Leaf<IT, LT> {
    pub fn new(value: Rc<RefCell<LeafImpl<IT, LT>>>) -> Self {
        Leaf { leaf: value }
    }

    pub fn value(&self) -> Ref<LT> {
        Ref::map(self.leaf.borrow(), |l| &l.value)
    }

    pub fn value_mut(&self) -> RefMut<LT> {
        RefMut::map(self.leaf.borrow_mut(), |l| &mut l.value)
    }

    pub fn remove_from_tree(self) -> Tree<IT, LT> {
        let parent = self.leaf.borrow_mut().parent.take();
        match &parent {
            None => {
                // The leaf has no parent already. Nothing to do here
            }
            Some(p) => {
                match p.upgrade() {
                    None => {}
                    Some(up) => {
                        up.borrow_mut().children.retain(|c| match c {
                            TreeElementImpl::Node(_) => {
                                // Removing leaf so nothing to do here
                                true
                            }
                            TreeElementImpl::Leaf(l) => !Rc::ptr_eq(l, &self.leaf),
                        })
                    }
                }
            }
        }
        Tree {
            tree: TreeElementImpl::Leaf(self.leaf),
        }
    }

    pub fn parent(&self) -> Option<Node<IT, LT>> {
        match &self.leaf.borrow().parent {
            None => None,
            Some(p) => Some(Node::new(p.upgrade().unwrap())),
        }
    }
}
