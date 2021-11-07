use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree_elements::tree_element::TreeElement;

#[derive(Debug)]
pub enum DLTreeError {
    // This error should never happen and is a bug in dltree
    // Please report any occurence of this error
    DoubleLinkIntegrityViolated,
    // This error happens if child manipulations like insertions or replacements are applied on the root element
    ChildOperationOnRootLevel,
}

pub enum Value<IT, LT> {
    Node(IT),
    Leaf(LT),
}

pub struct Tree<IT, LT> {
    pub(crate) tree: TreeElementImpl<IT, LT>,
}

impl<IT, LT> Tree<IT, LT> {
    pub fn new(root_value: Value<IT, LT>) -> Self {
        Tree {
            tree: TreeElementImpl::new(root_value, None),
        }
    }
    pub fn root_node(&self) -> TreeElement<IT, LT> {
        TreeElement::new(&self.tree)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::{DLTreeError, Tree, Value};
    use std::borrow::BorrowMut;

    #[test]
    fn tree_building_test() {
        let tree = Tree::new(Value::Node(23));
        assert!(tree.root_node().as_leaf().is_none());
        let mut node = tree.root_node().as_node().unwrap();
        assert_eq!(*node.value(), 23);

        let pushed_element = node.push_child(Value::Leaf(34));
        assert!(pushed_element.as_node().is_none());

        let leaf = pushed_element.as_leaf().unwrap();
        assert_eq!(*leaf.value(), 34);
        assert_eq!(node.children().len(), 1);
    }

    #[test]
    fn value_set_test() {
        let node = Tree::<i32, i32>::new(Value::Node(34));
        *node.root_node().borrow_mut().value_mut() = 33;
        assert_eq!(*node.root_node().borrow_mut().value(), 33);

        let leaf = Tree::<i32, i32>::new(Value::Leaf(45));
        *leaf.root_node().borrow_mut().value_mut() = 44;
        assert_eq!(*leaf.root_node().borrow_mut().value(), 44);
    }

    #[test]
    fn element_rempval_test() -> Result<(), DLTreeError> {
        let tree = Tree::new(Value::Node(11));
        let mut node11 = tree.root_node().as_node().unwrap();
        let mut node21 = node11.push_child(Value::Node(21)).as_node().unwrap();
        assert_eq!(node11, node21.parent()?.unwrap());
        let mut _node31 = node21.push_child(Value::Node(31)).as_node().unwrap();
        let mut node32 = node21.push_child(Value::Node(32)).as_node().unwrap();
        let mut _node33 = node21.push_child(Value::Node(33)).as_node().unwrap();
        let mut leaf34 = node21.push_child(Value::Leaf(34)).as_leaf().unwrap();
        let mut leaf_41 = node32.push_child(Value::Leaf(41)).as_leaf().unwrap();
        assert_eq!(leaf_41.parent()?.unwrap(), node32);

        let removed_node_32 = node32.remove_from_tree()?;
        {
            let n = removed_node_32.root_node().as_node().unwrap();
            assert_eq!(*n.value(), 32);
            assert_eq!(n.children().len(), 1);
            assert!(n.parent()?.is_none());
        }

        let _removed_leaf_34 = leaf34.remove_from_tree()?;

        assert_eq!(node21.children().len(), 2);
        assert_eq!(*node21.children().get(0).unwrap().value(), 31);
        assert_eq!(*node21.children().get(1).unwrap().value(), 33);

        {
            let l = leaf_41.remove_from_tree()?.root_node().as_leaf().unwrap();
            assert_eq!(*l.value(), 41);
            assert!(l.parent()?.is_none());
        }
        {
            let n = removed_node_32.root_node().as_node().unwrap();
            assert_eq!(n.children().len(), 0);
        }
        Ok(())
    }

    #[test]
    fn tree_element_test() -> Result<(), DLTreeError> {
        let tree = Tree::new(Value::Node(23));
        assert!(tree.root_node().parent()?.is_none());
        let mut node = tree.root_node().as_node().unwrap();
        let mut sub_tree = node.push_child(Value::Node(34));
        assert!(sub_tree.parent()?.is_some());
        let mut sub_sub_tree = sub_tree.as_node().unwrap().push_child(Value::Leaf(45));
        assert!(sub_sub_tree.parent()?.is_some());
        assert_eq!(sub_tree.as_node().unwrap().children().len(), 1);
        let removed_sub_sub_tree = sub_sub_tree.remove_from_tree()?;
        assert!(removed_sub_sub_tree.root_node().remove_from_tree().is_err());
        assert_eq!(sub_tree.as_node().unwrap().children().len(), 0);
        assert_eq!(tree.root_node().as_node().unwrap().children().len(), 1);
        let removed_sub_tree = sub_tree.remove_from_tree()?;
        assert!(removed_sub_tree.root_node().remove_from_tree().is_err());
        assert_eq!(tree.root_node().as_node().unwrap().children().len(), 0);
        Ok(())
    }

    #[test]
    fn replace_test() -> Result<(), DLTreeError> {
        let tree = Tree::<i32, i32>::new(Value::Node(34));
        let leaf = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_child(Value::Leaf(45));
        let mut node = leaf.as_leaf().unwrap().replace_with_node(56)?;
        node.push_child(Value::Leaf(67));
        let sub_leaf = node.replace_with_leaf(78)?;
        assert_eq!(*sub_leaf.parent()?.unwrap().value(), 34);
        Ok(())
    }
}
