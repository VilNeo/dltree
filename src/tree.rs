use crate::internal::tree_element_impl::TreeElementImpl;
use crate::tree_elements::tree_element::TreeElement;

#[derive(Debug)]
pub enum DLTreeError {
    // This error should never happen and is a bug in dltree
    // Please report any occurence of this error
    IntegrityViolated,
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
    use crate::DeepClone;
    use std::borrow::BorrowMut;

    #[test]
    fn tree_building_test() {
        let tree = Tree::new(Value::Node(23));
        assert!(tree.root_node().as_leaf().is_none());
        let mut node = tree.root_node().clone().as_node().unwrap();
        assert_eq!(*node.value(), 23);

        let pushed_element = node.push_back_child(Value::Leaf(34));
        assert!(pushed_element.as_node().is_none());

        let leaf = pushed_element.clone().as_leaf().unwrap();
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
    fn element_removal_test() -> Result<(), DLTreeError> {
        let tree = Tree::new(Value::Node(11));
        let mut node11 = tree.root_node().as_node().unwrap();
        let mut node21 = node11.push_back_child(Value::Node(21)).as_node().unwrap();
        assert_eq!(node11, node21.parent()?.unwrap());
        let mut _node31 = node21.push_back_child(Value::Node(31)).as_node().unwrap();
        let mut node32 = node21.push_back_child(Value::Node(32)).as_node().unwrap();
        let mut _node33 = node21.push_back_child(Value::Node(33)).as_node().unwrap();
        let mut leaf34 = node21.push_back_child(Value::Leaf(34)).as_leaf().unwrap();
        let mut leaf_41 = node32.push_back_child(Value::Leaf(41)).as_leaf().unwrap();
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
        let mut sub_tree = node.push_back_child(Value::Node(34));
        assert!(sub_tree.parent()?.is_some());
        let mut sub_sub_tree = sub_tree.as_node().unwrap().push_back_child(Value::Leaf(45));
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
        let mut leaf = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(45));
        let mut node = leaf.set(Value::Node(56))?;
        node.as_node().unwrap().push_back_child(Value::Leaf(67));
        let sub_leaf = node.set(Value::Leaf(78))?.as_leaf().unwrap();
        assert_eq!(*sub_leaf.parent()?.unwrap().value(), 34);
        Ok(())
    }
    #[test]
    fn replace_leaf_test() -> Result<(), DLTreeError> {
        let tree = Tree::<i32, i32>::new(Value::Node(34));
        let mut leaf = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(45));
        let mut leaf2 = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(46));
        let mut node = leaf.set(Value::Node(56))?;
        node.as_node().unwrap().push_back_child(Value::Leaf(67));
        let sub_leaf = node.set_leaf(78)?;
        assert_eq!(*sub_leaf.parent()?.unwrap().value(), 34);

        let replaced_leaf2 = leaf2.set_leaf(146)?;
        assert_eq!(*replaced_leaf2.parent()?.unwrap().value(), 34);
        Ok(())
    }
    #[test]
    fn replace_node_test() -> Result<(), DLTreeError> {
        let tree = Tree::<i32, i32>::new(Value::Node(34));
        let leaf = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(45));
        let mut leaf2 = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(46));
        let mut node = leaf.as_leaf().unwrap().set(Value::Node(56))?;
        node.as_node().unwrap().push_back_child(Value::Leaf(67));
        let sub_node = node.set_node(78)?;
        assert_eq!(*sub_node.parent()?.unwrap().value(), 34);

        let replaced_leaf2 = leaf2.set_node(146)?;
        assert_eq!(*replaced_leaf2.parent()?.unwrap().value(), 34);
        Ok(())
    }
    #[test]
    fn insert_test() -> Result<(), DLTreeError> {
        let tree = Tree::<i32, i32>::new(Value::Node(11));
        tree.root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(21));
        let middle_leaf = tree
            .root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(22));
        tree.root_node()
            .as_node()
            .unwrap()
            .push_back_child(Value::Leaf(23));
        assert_eq!(
            tree.root_node()
                .as_node()
                .unwrap()
                .children()
                .iter()
                .map(|c| *c.value())
                .collect::<Vec<i32>>(),
            vec![21, 22, 23]
        );
        middle_leaf
            .as_leaf()
            .unwrap()
            .insert_before(Value::Leaf(44))?;
        middle_leaf
            .as_leaf()
            .unwrap()
            .insert_after(Value::Node(45))?;
        assert_eq!(
            tree.root_node()
                .as_node()
                .unwrap()
                .children()
                .iter()
                .map(|c| *c.value())
                .collect::<Vec<i32>>(),
            vec![21, 44, 22, 45, 23]
        );
        Ok(())
    }

    #[test]
    fn remove_all_children_test() -> Result<(), DLTreeError> {
        let tree = Tree::new(Value::Node(23));
        let mut root_node = tree.root_node().as_node().unwrap();
        root_node.push_back_child(Value::Leaf(1));
        root_node.push_back_child(Value::Node(2));
        root_node.push_back_child(Value::Leaf(3));
        root_node.push_back_child(Value::Node(4));
        assert_eq!(root_node.children().len(), 4);
        let children = root_node.children();
        root_node.remove_all_children()?;
        assert_eq!(root_node.children().len(), 0);
        assert_eq!(
            children
                .iter()
                .filter(|c| c.parent().unwrap().is_none())
                .count(),
            4
        );
        Ok(())
    }

    #[test]
    fn deep_clone_test() -> Result<(), DLTreeError> {
        // Create a new tree with three levels under the root node
        // Clone a subtree under the root node
        // Set new values in the cloned tree
        // 1. Check integrity (all parents must be set properly)
        // 2. Check for separateness (new cloned tree must not share any node with the original tree)

        let tree = Tree::new(Value::Node(23));
        let mut root_node = tree.root_node().clone().as_node().unwrap();

        let _child_1a = root_node.push_back_child(Value::Leaf(11));
        let mut child_1b = root_node
            .push_back_child(Value::Node(12))
            .as_node()
            .unwrap();
        let _child_1c = root_node.push_back_child(Value::Leaf(13));

        let child_2a = child_1b.push_back_child(Value::Leaf(21));
        let child_2b = child_1b.push_back_child(Value::Leaf(22));
        let mut child_2c = child_1b.push_back_child(Value::Node(23)).as_node().unwrap();

        let child_3a = child_2c.push_back_child(Value::Leaf(31));
        let child_3b = child_2c.push_back_child(Value::Node(32));

        let cloned_node = child_1b.deep_clone();
        assert!(cloned_node.parent()?.is_none());
        *cloned_node.value_mut() = 112;
        assert_eq!(*child_1b.value(), 12);
        assert_eq!(*cloned_node.value(), 112);

        assert_eq!(cloned_node.children().len(), 3);
        let cloned_child_2a = cloned_node.children().get(0).unwrap().as_leaf().unwrap();
        let cloned_child_2b = cloned_node.children().get(1).unwrap().as_leaf().unwrap();
        let cloned_child_2c = cloned_node.children().get(2).unwrap().as_node().unwrap();

        *cloned_child_2a.value_mut() = 121;
        *cloned_child_2b.value_mut() = 122;
        *cloned_child_2c.value_mut() = 123;
        assert_eq!(*child_2a.value(), 21);
        assert_eq!(*child_2b.value(), 22);
        assert_eq!(*child_2c.value(), 23);
        assert_eq!(*cloned_child_2a.value(), 121);
        assert_eq!(*cloned_child_2b.value(), 122);
        assert_eq!(*cloned_child_2c.value(), 123);
        assert_eq!(*cloned_child_2a.parent()?.unwrap().value(), 112);
        assert_eq!(*cloned_child_2b.parent()?.unwrap().value(), 112);
        assert_eq!(*cloned_child_2c.parent()?.unwrap().value(), 112);

        assert_eq!(cloned_child_2c.children().len(), 2);
        let cloned_child_3a = cloned_child_2c
            .children()
            .get(0)
            .unwrap()
            .as_leaf()
            .unwrap();
        let cloned_child_3b = cloned_child_2c
            .children()
            .get(1)
            .unwrap()
            .as_node()
            .unwrap();

        *cloned_child_3a.value_mut() = 131;
        *cloned_child_3b.value_mut() = 132;
        assert_eq!(*child_3a.value(), 31);
        assert_eq!(*child_3b.value(), 32);
        assert_eq!(*cloned_child_3a.value(), 131);
        assert_eq!(*cloned_child_3b.value(), 132);
        assert_eq!(*cloned_child_3a.parent()?.unwrap().value(), 123);
        assert_eq!(*cloned_child_3b.parent()?.unwrap().value(), 123);

        Ok(())
    }
}
