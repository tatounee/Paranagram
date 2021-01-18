#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod utils;
mod word;

use utils::*;
use word::Word;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::cmp::min;

const PARANAGRAM_MAX_DEEP: usize = 10;

pub struct Paranagram {
    path_data: String,
    sacamot: Vec<String>,
}

impl Paranagram {
    pub fn new(path_data: &str) -> io::Result<Self> {
        // Open and read the data file
        let path = Path::new(path_data);
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;


        // Parse the content of the data file to create an vec of all word
        let mut sacamot: Vec<String> = buffer
            .lines()
            .filter_map(|s| {
                let s = s.trim_end().to_owned();
                if s.len() != 0 {
                    Some(s)
                } else {
                    None
                }

            })
            .collect();

        // Return our Paranagram
        Ok(Self {
            path_data: path_data.to_owned(),
            sacamot,
        })
    }

    fn existing_anagrams(&self, sentence: &str) -> Vec<&String> {
        let trie = Trie::new_with_iter_and_maximun_deep(sentence.chars(), min(self.max_len, PARANAGRAM_MAX_DEEP));
        self.sacamot.iter().flat_map(|word| {
            if trie.existing(word.chars()) {
                Some(word)
            } else {
                None
            }
        }).collect::<Vec<&String>>()
    }

}

impl fmt::Debug for Paranagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't print the field "sacamot" because it's too large an uninteresting
        f.debug_struct("Paranagram")
            .field("path_data", &self.path_data)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let paranagram = Paranagram::new("data/word.txt");
        println!("{:?}", paranagram);
    }

    #[test]
    fn find_all_anagram_of_a_word() {
        let word = "Jean le parisien";
        let paranagram = Paranagram::new("data/word.txt").unwrap();
        let anagrams = paranagram.existing_anagrams(word);
        println!("{:?}", anagrams);
    }
}
