use super::tree_element::TreeElement;
use crate::internal::node_impl::NodeImpl;
use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree::{Tree, Value};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<IT, LT> {
    node: Rc<RefCell<NodeImpl<IT, LT>>>,
}

impl<IT, LT> PartialEq for Node<IT, LT> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
    }
}

impl<IT, LT> Clone for Node<IT, LT> {
    fn clone(&self) -> Self {
        Node {
            node: self.node.clone(),
        }
    }
}

impl<IT, LT> Node<IT, LT> {
    pub fn new(value: Rc<RefCell<NodeImpl<IT, LT>>>) -> Self {
        Node { node: value }
    }

    pub fn value(&self) -> Ref<IT> {
        Ref::map(self.node.borrow(), |n| &n.value)
    }

    pub fn value_mut(&mut self) -> RefMut<IT> {
        RefMut::map(self.node.borrow_mut(), |n| &mut n.value)
    }

    pub fn push_child(&mut self, value: Value<IT, LT>) -> TreeElement<IT, LT> {
        let new_child = TreeElementImpl::new(value, Some(Rc::downgrade(&self.node)));
        let result = TreeElement::new(&new_child);
        self.node.borrow_mut().children.push(new_child);
        result
    }

    pub fn children(&self) -> Vec<TreeElement<IT, LT>> {
        self.node
            .borrow()
            .children
            .iter()
            .map(|c| TreeElement::new(c))
            .collect()
    }

    pub fn remove_from_tree(self) -> Tree<IT, LT> {
        let parent = self.node.borrow_mut().parent.take();
        match &parent {
            None => {
                // The node has no parent already. Nothing to do here
            }
            Some(p) => {
                match p.upgrade() {
                    None => {}
                    Some(up) => {
                        up.borrow_mut().children.retain(|c| match c {
                            TreeElementImpl::Node(n) => !Rc::ptr_eq(n, &self.node),
                            TreeElementImpl::Leaf(_) => true, // Removing node so nothing to do here
                        })
                    }
                }
            }
        }
        Tree {
            tree: TreeElementImpl::Node(self.node),
        }
    }

    pub fn parent(&self) -> Option<Node<IT, LT>> {
        match &self.node.borrow().parent {
            None => None,
            Some(p) => Some(Node::new(p.upgrade().unwrap())),
        }
    }
}
