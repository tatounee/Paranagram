mod symetrical_tree_node;

use symetrical_tree_node::TreeNode;

use std::ops::Index;

#[derive(Debug)]
pub(crate) struct SymetricalTree<K> {
    nodes: Vec<TreeNode<K>>,
}

impl<K> SymetricalTree<K> {
    pub(crate) fn new() -> Self {
        Self { nodes: vec![]}
    }

    pub(crate) fn new_from_iter<I>(data: I) -> Self
    where
        I: Iterator<Item = K>,
        K: PartialEq + Copy,
    {
        let mut nodes = vec![];

        let vec_data = data.collect::<Vec<K>>();

        for i in 0..vec_data.len() {
            let mut new_data = vec_data.clone();
            let key = new_data.remove(i);
            nodes.push(TreeNode::new_from_slice(key, &mut new_data[..]))
        }

        Self{
            nodes,
        }
    }

    pub(crate) fn existing<I>(&self, sentence: I) -> bool
    where
        I: Iterator<Item = K>,
        K: PartialEq,
    {
        let mut current_node = Node::Root(self);

        for fragment in sentence {
            let mut tmp: Node<K>;
            match current_node.get_node(fragment) {
                Some(node) => current_node = Node::Node(&node),
                None => return false,
            }
        }
        true
    }

}


enum Node<'a, K>{
    Root(&'a SymetricalTree<K>),
    Node(&'a TreeNode<K>)
}

impl<'a, K> Node<'a, K> {
    fn get_node(&self, index: K) -> Option<TreeNode<K>>
    where
        K: PartialEq
    {
        match self {
            Node::Root(r) => {
                let i  = r.nodes.into_iter().position(|n| n.key == index)?;
                r.nodes.index(i)
            },
            Node::Node(n) => n.nodes.into_iter().find(|n| n.key == index),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_tree() {
        let tree: SymetricalTree<char> = SymetricalTree::new();
    }

    #[test]
    fn new_tree_from_str() {
        let tree: SymetricalTree<char> = SymetricalTree::new_from_iter("0123".chars());
        println!("{:?}\n", tree);
        println!("{:#?}", tree);
    }
}
