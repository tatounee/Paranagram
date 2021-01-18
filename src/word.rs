use crate::{hashmap, utils::*};

use std::collections::HashMap;
use unidecode::unidecode;

#[derive(Debug)]
pub(crate) struct Word {
    pub(crate) word: String,
    pub(crate) letters: HashMap<char, u16>,
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
        self.word.len()
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
