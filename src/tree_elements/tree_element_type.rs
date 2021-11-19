use crate::internal::leaf_impl::LeafImpl;
use crate::internal::node_impl::NodeImpl;
use crate::internal::tree_element_impl::TreeElementImpl;
use crate::internal::TreeElementTrait;
use crate::tree::{DLTreeError, Tree, Value};
use crate::tree_elements::tree_element::TreeElement;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct TreeElementType<IT, LT, T: TreeElementTrait<IT, LT>> {
    leaf: Rc<RefCell<T>>,
    phantom_it: PhantomData<IT>,
    phantom_lt: PhantomData<LT>,
}

pub type Node<IT, LT> = TreeElementType<IT, LT, NodeImpl<IT, LT>>;
pub type Leaf<IT, LT> = TreeElementType<IT, LT, LeafImpl<IT, LT>>;

impl<IT, LT, T: TreeElementTrait<IT, LT>> TreeElementType<IT, LT, T> {
    pub fn new(value: Rc<RefCell<T>>) -> Self {
        TreeElementType {
            leaf: value,
            phantom_it: PhantomData::default(),
            phantom_lt: PhantomData::default(),
        }
    }

    fn is_same(&self, other: &TreeElementImpl<IT, LT>) -> bool {
        let other_ptr = match other {
            TreeElementImpl::Node(n) => n.as_ptr() as *mut T,
            TreeElementImpl::Leaf(l) => l.as_ptr() as *mut T,
        };
        self.leaf.as_ptr() == other_ptr
    }

    fn update_as_child<F, R>(&self, update_fn: F) -> Result<R, DLTreeError>
    where
        F: FnOnce(
            usize,
            &mut Vec<TreeElementImpl<IT, LT>>,
            Weak<RefCell<NodeImpl<IT, LT>>>,
        ) -> Result<R, DLTreeError>,
    {
        let parent = self
            .parent()?
            .ok_or(DLTreeError::ChildOperationOnRootLevel)?
            .leaf;
        let index = parent
            .borrow()
            .children
            .iter()
            .enumerate()
            .find(|(_, child)| self.is_same(child))
            .ok_or(DLTreeError::IntegrityViolated)?
            .0;
        return update_fn(
            index,
            &mut parent.borrow_mut().children,
            Rc::downgrade(&parent),
        );
    }

    pub fn parent(&self) -> Result<Option<Node<IT, LT>>, DLTreeError> {
        match &self.leaf.borrow_mut().parent() {
            None => Ok(None),
            Some(p) => match p.upgrade() {
                None => {
                    // Not being able to upgrade the weak pointer to parent can only have one cause:
                    // This node has been removed from parent node but the weak pointer to the
                    // parrent node has not been set to 'None'
                    // This should never happen.
                    Err(DLTreeError::IntegrityViolated)
                }
                Some(upgraded_p) => Ok(Some(TreeElementType::new(upgraded_p))),
            },
        }
    }
    pub fn set(&mut self, value: Value<IT, LT>) -> Result<TreeElement<IT, LT>, DLTreeError> {
        self.update_as_child(|index, children, parent| {
            let child = children
                .get_mut(index)
                .ok_or(DLTreeError::IntegrityViolated)?;
            match value {
                Value::Node(n) => {
                    let node = Rc::new(RefCell::new(NodeImpl::new(n, Some(parent))));
                    *child = TreeElementImpl::Node(node.clone());
                    Ok(TreeElement::new(child))
                }
                Value::Leaf(l) => {
                    let leaf = Rc::new(RefCell::new(LeafImpl::new(l, Some(parent))));
                    *child = TreeElementImpl::Leaf(leaf.clone());
                    Ok(TreeElement::new(child))
                }
            }
        })
    }

    pub fn set_leaf(&mut self, value: LT) -> Result<Leaf<IT, LT>, DLTreeError> {
        self.update_as_child(|index, children, parent| {
            let child = children
                .get_mut(index)
                .ok_or(DLTreeError::IntegrityViolated)?;
            let leaf = Rc::new(RefCell::new(LeafImpl::new(value, Some(parent))));
            *child = TreeElementImpl::Leaf(leaf.clone());
            Ok(Leaf {
                leaf,
                phantom_it: Default::default(),
                phantom_lt: Default::default(),
            })
        })
    }

    pub fn set_node(&mut self, value: IT) -> Result<Node<IT, LT>, DLTreeError> {
        self.update_as_child(|index, children, parent| {
            let child = children
                .get_mut(index)
                .ok_or(DLTreeError::IntegrityViolated)?;
            let node = Rc::new(RefCell::new(NodeImpl::new(value, Some(parent))));
            *child = TreeElementImpl::Node(node.clone());
            Ok(Node {
                leaf: node,
                phantom_it: Default::default(),
                phantom_lt: Default::default(),
            })
        })
    }

    pub fn insert_before(&mut self, value: Value<IT, LT>) -> Result<Tree<IT, LT>, DLTreeError> {
        let inserted = self.update_as_child(|index, children, parent| {
            let new_element = TreeElementImpl::new(value, Some(parent));
            children.insert(index, new_element.clone());
            Ok(new_element)
        })?;
        Ok(Tree { tree: inserted })
    }

    pub fn insert_after(&mut self, value: Value<IT, LT>) -> Result<Tree<IT, LT>, DLTreeError> {
        let inserted = self.update_as_child(|index, children, parent| {
            let new_element = TreeElementImpl::new(value, Some(parent));
            children.insert(index + 1, new_element.clone());
            Ok(new_element)
        })?;
        Ok(Tree { tree: inserted })
    }

    pub fn remove_from_tree(&mut self) -> Result<Tree<IT, LT>, DLTreeError> {
        let removed_child = self.update_as_child(|index, children, _| {
            let removed_child = children.remove(index);
            *self.leaf.borrow_mut().parent() = None;
            Ok(removed_child)
        })?;
        Ok(Tree {
            tree: removed_child,
        })
    }
}

impl<IT, LT, T: TreeElementTrait<IT, LT>> Clone for TreeElementType<IT, LT, T> {
    fn clone(&self) -> Self {
        TreeElementType::new(self.leaf.clone())
    }
}

impl<IT, LT, T: TreeElementTrait<IT, LT>> PartialEq for TreeElementType<IT, LT, T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.leaf, &other.leaf)
    }
}

impl<IT, LT> Leaf<IT, LT> {
    pub fn value(&self) -> Ref<LT> {
        Ref::map(self.leaf.borrow(), |l| &l.value)
    }

    pub fn value_mut(&self) -> RefMut<LT> {
        RefMut::map(self.leaf.borrow_mut(), |l| &mut l.value)
    }
}

impl<IT, LT> Node<IT, LT> {
    pub fn push_child(&mut self, value: Value<IT, LT>) -> TreeElement<IT, LT> {
        let new_child = TreeElementImpl::new(value, Some(Rc::downgrade(&self.leaf)));
        let result = TreeElement::new(&new_child);
        self.leaf.borrow_mut().children.push(new_child);
        result
    }
    pub fn push_child_tree(&mut self, subtree: TreeElement<IT, LT>) -> TreeElement<IT, LT> {
        let new_child = match subtree {
            TreeElement::Node(n) => {
                n.leaf.borrow_mut().parent = Some(Rc::downgrade(&self.leaf));
                TreeElementImpl::Node(n.leaf)
            }
            TreeElement::Leaf(l) => {
                l.leaf.borrow_mut().parent = Some(Rc::downgrade(&self.leaf));
                TreeElementImpl::Leaf(l.leaf)
            }
        };
        let result = TreeElement::new(&new_child);
        self.leaf.borrow_mut().children.push(new_child);
        result
    }
    pub fn remove_all_children(&mut self) -> Result<(), DLTreeError> {
        let mut removed_children = vec![];
        while let Some(mut child) = self.leaf.borrow_mut().children.pop() {
            match &mut child {
                TreeElementImpl::Node(n) => n.borrow_mut().parent = None,
                TreeElementImpl::Leaf(l) => l.borrow_mut().parent = None,
            }
            removed_children.push(child);
        }
        assert!(self.leaf.borrow_mut().children.is_empty());
        Ok(())
    }
    pub fn children(&self) -> Vec<TreeElement<IT, LT>> {
        self.leaf
            .borrow()
            .children
            .iter()
            .map(|c| TreeElement::new(c))
            .collect()
    }

    pub fn value(&self) -> Ref<IT> {
        Ref::map(self.leaf.borrow(), |l| &l.value)
    }

    pub fn value_mut(&self) -> RefMut<IT> {
        RefMut::map(self.leaf.borrow_mut(), |l| &mut l.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::node_impl::NodeImpl;

    #[test]
    fn check_integrity_violation() {
        let mut parent_node = Rc::new(RefCell::new(NodeImpl::new(21, None)));
        let mut leaf = Leaf::new(Rc::new(RefCell::new(LeafImpl::new(
            32,
            Some(Rc::downgrade(&parent_node)),
        ))));
        parent_node = Rc::new(RefCell::new(NodeImpl::new(43, None)));
        assert!(leaf.parent().is_err());
        assert!(leaf.remove_from_tree().is_err());
        assert_eq!(parent_node.borrow().value, 43);
    }
}
