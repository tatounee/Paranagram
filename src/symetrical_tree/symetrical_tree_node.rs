
use std::ops::Index;
use super::Node;
#[derive(Debug)]
pub(super) struct TreeNode<K> {
    pub(super) key: K,
    pub(super) nodes: Vec<TreeNode<K>>,
}

impl<K> TreeNode<K> {
    pub(super) fn new(key: K) -> Self {
        Self {
            key,
            nodes: vec![],
        }
    }

    pub(super) fn insert(&mut self, node: Self) {
        self.nodes.push(node);
    }

    pub(crate) fn new_from_slice(key: K, data: &mut [K]) -> Self
    where
        K: Clone + Copy
    {
        let mut node = TreeNode::new(key);

        if data.len() == 1 {
            node.insert(Self::new(data[0]));
            return node
        }

        for i in 0..data.len() {
            let mut new_data = data.to_vec();
            let key = new_data.remove(i);
            node.insert(TreeNode::new_from_slice(key, &mut new_data[..]))
        }
        node
    }
}