use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree::{DLTreeError, Tree, Value};
use crate::tree_elements::tree_element_type::{Leaf, Node};
use crate::DeepClone;
use std::cell::{Ref, RefMut};

#[derive(Debug)]
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
    pub fn parent(&self) -> Result<Option<Node<IT, LT>>, DLTreeError> {
        match self {
            TreeElement::Node(n) => n.parent(),
            TreeElement::Leaf(l) => l.parent(),
        }
    }
    pub fn remove_from_tree(&mut self) -> Result<Tree<IT, LT>, DLTreeError> {
        match self {
            TreeElement::Node(n) => n.remove_from_tree(),
            TreeElement::Leaf(l) => l.remove_from_tree(),
        }
    }
    pub fn set(&mut self, value: Value<IT, LT>) -> Result<TreeElement<IT, LT>, DLTreeError> {
        match self {
            TreeElement::Node(n) => n.set(value),
            TreeElement::Leaf(l) => l.set(value),
        }
    }
    pub fn set_leaf(&mut self, value: LT) -> Result<Leaf<IT, LT>, DLTreeError> {
        match self {
            TreeElement::Node(n) => n.set_leaf(value),
            TreeElement::Leaf(l) => l.set_leaf(value),
        }
    }
    pub fn set_node(&mut self, value: IT) -> Result<Node<IT, LT>, DLTreeError> {
        match self {
            TreeElement::Node(n) => n.set_node(value),
            TreeElement::Leaf(l) => l.set_node(value),
        }
    }
}

impl<IT, LT> Clone for TreeElement<IT, LT> {
    fn clone(&self) -> Self {
        match &self {
            TreeElement::Node(n) => TreeElement::Node(n.clone()),
            TreeElement::Leaf(l) => TreeElement::Leaf(l.clone()),
        }
    }
}

impl<IT: Clone, LT: Clone> DeepClone for TreeElement<IT, LT> {
    fn deep_clone(&self) -> Self {
        match self {
            TreeElement::Node(n) => TreeElement::Node(n.deep_clone()),
            TreeElement::Leaf(l) => TreeElement::Leaf(l.deep_clone()),
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
