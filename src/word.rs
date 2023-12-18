use crate::utils::*;

use unidecode::unidecode;

use std::collections::HashMap;
use std::fmt;

pub struct Word {
    word: String,
    len: usize,
    weight: usize,
    letters: HashMap<char, u16>,
}

impl Word {
    pub fn new(word: &str) -> Self {
        let word = word
            .chars()
            .filter(|c| c.is_alphabetic() || c == &'-')
            .collect::<String>();

        let letters = unidecode(&word)
            .chars()
            .filter(|c| c != &'-')
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>()
            .to_hashmap();

        let weight = letters
            .iter()
            .map(|(k, v)| ((*k as u8 - 96) as u16 * *v) as usize)
            .sum::<usize>();

        Self {
            len: unidecode(&word).len(),
            word,
            weight,
            letters,
        }
    }

    #[inline]
    pub fn contains(&self, word: &Word) -> bool {
        self.letters.contains(&word.letters)
    }

    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn weight(&self) -> usize {
        self.weight
    }

    #[inline]
    pub fn letters(&self) -> &HashMap<char, u16> {
        &self.letters
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

#[derive(Debug, Clone)]
pub struct IndexAndWeight {
    pub index: usize,
    pub weight: usize,
}

impl IndexAndWeight {
    pub fn new(index: usize, weight: usize) -> Self {
        Self { index, weight }
    }
}

pub trait ToIndexAndWeight {
    fn to_index_and_weight(&self) -> Vec<IndexAndWeight>;
}

pub trait FromIndexAndWeight {
    fn from_index_and_weight(&self, index_and_weight: Vec<&IndexAndWeight>) -> Self;
}

impl ToIndexAndWeight for Vec<&Word> {
    #[inline]
    fn to_index_and_weight(&self) -> Vec<IndexAndWeight> {
        self.iter()
            .enumerate()
            .map(|(i, w)| IndexAndWeight::new(i, w.weight()))
            .collect()
    }
}

impl FromIndexAndWeight for Vec<&Word> {
    #[inline]
    fn from_index_and_weight(&self, index_and_weight: Vec<&IndexAndWeight>) -> Self {
        index_and_weight.iter().map(|x| *self.get(x.index).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashmap;
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

    #[test]
    fn len_unidecode() {
        let a = Word::new("à");
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn weight_char() {
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                let mut b = [0; 2];
                let s = c.encode_utf8(&mut b);
                assert_eq!(Word::new(s).weight(), i + 1);
            });
    }

    #[test]
    fn weight_word_with_unique_chars() {
        let w = Word::new("abcde");
        assert_eq!(w.weight(), 15);
    }

    #[test]
    fn weight_word_with_multiple_chars() {
        let w = Word::new("aàbcdee");
        assert_eq!(w.weight(), 21);
    }
}
