mod internal;
pub mod tree;
pub mod tree_elements;

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}
