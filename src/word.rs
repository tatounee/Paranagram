
use crate::{hashmap, utils::*};

use unidecode::unidecode;

use std::collections::HashMap;
use std::fmt;

// #[derive(Debug)]
pub struct Word {
    word: String,
    len: usize,
    weight: usize,
    letters: HashMap<char, u16>,
}

impl Word {
    pub(crate) fn new(word: &str) -> Self {
        let word = word
            .chars()
            .filter(|c| c.is_alphabetic() || c == &'-')
            .collect::<String>();
        let hashable = unidecode(&word)
            .chars()
            .filter(|c| !(c == &'-'))
            .collect::<String>();
        Self {
            letters: hashable.to_hashmap(),
            word,
        }
    }

    #[inline]
    pub(crate) fn contains(&self, word: &Word) -> bool {
        self.letters.contains(&word.letters)
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len
    }
    
    #[inline]
    pub(crate) fn weight(&self) -> usize {
        self.weight
    }   
    
    #[inline]
    pub(crate) fn letters(&self) -> &HashMap<char, u16> {
        &self.letters
    }

    #[cfg(test)]
    pub(crate) fn new_perso(word: String, len: usize, weight: usize, letters: HashMap<char, u16>) -> Self {
        Self{word, len, weight, letters}
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Word")
            .field("len", &self.len)
            .field("weight", &self.weight)
            .finish()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_accent() {
        let w = Word::new("éeèaà");
        assert_eq!(w.word, "éeèaà");
        assert_eq!(w.letters, hashmap!['e' => 3, 'a' => 2]);
    }

    #[test]
    fn retains_letters() {
        let w = Word::new("a(- 1a");
        assert_eq!(w.word, "a-a");
        assert_eq!(w.letters, hashmap!['a' => 2])
    }
}
