use super::{leaf::Leaf, node::Node};
use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree::Tree;
use std::cell::{Ref, RefMut};

pub enum TreeElement<IT, LT> {
    Node(Node<IT, LT>),
    Leaf(Leaf<IT, LT>),
}

impl<IT, LT> TreeElement<IT, LT> {
    pub fn new(value: &TreeElementImpl<IT, LT>) -> Self {
        match value {
            TreeElementImpl::Node(n) => TreeElement::Node(Node::new(n.clone())),
            TreeElementImpl::Leaf(l) => TreeElement::Leaf(Leaf::new(l.clone())),
        }
    }
    pub fn as_node(&self) -> Option<Node<IT, LT>> {
        match self {
            TreeElement::Node(n) => Some(n.clone()),
            TreeElement::Leaf(_) => None,
        }
    }
    pub fn as_leaf(&self) -> Option<Leaf<IT, LT>> {
        match self {
            TreeElement::Node(_) => None,
            TreeElement::Leaf(l) => Some(l.clone()),
        }
    }
    pub fn parent(&self) -> Option<Node<IT, LT>> {
        match &self {
            TreeElement::Node(n) => n.parent(),
            TreeElement::Leaf(l) => l.parent(),
        }
    }
    pub fn remove_from_tree(self) -> Tree<IT, LT> {
        match self {
            TreeElement::Node(n) => n.remove_from_tree(),
            TreeElement::Leaf(l) => l.remove_from_tree(),
        }
    }
}

impl<T> TreeElement<T, T> {
    pub fn value(&self) -> Ref<T> {
        match self {
            TreeElement::Node(n) => n.value(),
            TreeElement::Leaf(l) => l.value(),
        }
    }
    pub fn value_mut(&mut self) -> RefMut<T> {
        match self {
            TreeElement::Node(n) => n.value_mut(),
            TreeElement::Leaf(l) => l.value_mut(),
        }
    }
}
