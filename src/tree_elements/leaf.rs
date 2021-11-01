use super::node::Node;
use crate::internal::leaf_impl::LeafImpl;
use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree::{DLTreeError, Tree};
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

    pub fn remove_from_tree(&mut self) -> Result<Tree<IT, LT>, DLTreeError> {
        let parent_node = match &self.leaf.borrow_mut().parent {
            None => {
                return Ok(Tree {
                    tree: TreeElementImpl::Leaf(self.leaf.clone()),
                })
            }
            Some(p) => match p.upgrade() {
                None => return Err(DLTreeError::ParentAlreadyInUse),
                Some(upgraded_p) => upgraded_p,
            },
        };
        parent_node.borrow_mut().children.retain(|c| match c {
            TreeElementImpl::Node(_) => true, // Removing leaf so nothing to do here
            TreeElementImpl::Leaf(l) => !Rc::ptr_eq(l, &self.leaf),
        });
        self.leaf.borrow_mut().parent = None;
        Ok(Tree {
            tree: TreeElementImpl::Leaf(self.leaf.clone()),
        })
    }

    pub fn parent(&self) -> Result<Option<Node<IT, LT>>, DLTreeError> {
        match &self.leaf.borrow().parent {
            None => Ok(None),
            Some(p) => match p.upgrade() {
                None => Err(DLTreeError::ParentAlreadyInUse),
                Some(upgraded_p) => Ok(Some(Node::new(upgraded_p))),
            },
        }
    }
}
