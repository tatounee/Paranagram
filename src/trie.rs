
use std::collections::HashMap;
use std::hash;

use rayon::prelude::*;

#[derive(Debug)]
pub(crate) struct Trie {
    nodes: HashMap<char, Trie>,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Self { nodes: HashMap::new()}
    }

    pub(crate) fn new_with_iter<I>(data: I) -> Self
    where
        I: Iterator<Item = char>,
    {
        let mut nodes = HashMap::new();

        let vec_data = data.into_iter().collect::<Vec<char>>();

        for i in 0..vec_data.len() {
            let mut new_data = vec_data.clone();
            let key = new_data.remove(i);
            nodes.insert(key, Trie::new_with_iter(new_data.into_iter()));
        }

        Self{
            nodes,
        }
    }    
    
    pub(crate) fn new_with_iter_and_maximun_deep<I>(data: I, maximun_deep: usize) -> Self
    where
        I: Iterator<Item = char>,
    {
        if maximun_deep == 0 {
            return Self::new();
        }

        let vec_data = data.into_iter().collect::<Vec<char>>();

        let nodes = (0..vec_data.len()).into_par_iter().map(|i| {
            let mut new_data = vec_data.clone();
            let key = new_data.remove(i);
            (key, Trie::new_with_iter_and_maximun_deep(new_data.into_iter(), maximun_deep - 1))
        }).collect::<HashMap<char, Trie>>();

        Self{
            nodes,
        }
    }

    pub(crate) fn existing<I>(&self, sentence: I) -> bool
    where
        I: Iterator<Item = char>,
    {
        let mut current_node = self;

        for fragment in sentence {
            match current_node.nodes.get(&fragment) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        true
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_tree() {
        let tree = Trie::new();
    }

    #[test]
    fn new_tree_from_str() {
        let trie = Trie::new_with_iter("012".chars());
        // println!("{:#?}", trie);
    }

    #[test]
    fn new_tree_from_str_and_maximun_deep() {
        let trie = Trie::new_with_iter_and_maximun_deep("0123".chars(), 3);
        // println!("{:#?}", trie);
    }

    #[test]
    fn word_existing() {
        let trie = Trie::new_with_iter("01233".chars());
        assert!(trie.existing("32130".chars()));
        assert!(trie.existing("0123".chars()));
        assert!(trie.existing("203".chars()));
        assert!(trie.existing("32".chars()));
        assert!(trie.existing("1".chars()));
    }    
    
    #[test]
    fn word_not_existing() {
        let trie = Trie::new_with_iter("0123".chars());
        assert!(!trie.existing("01230".chars()));
        assert!(!trie.existing("3200".chars()));
        assert!(!trie.existing("412".chars()));
        assert!(!trie.existing("125".chars()));
    }
}
