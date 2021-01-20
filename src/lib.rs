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
use std::collections::HashMap;

use rayon::prelude::*;


pub struct Paranagram {
    path_data: String,
    sacamot: Vec<Word>,
}

impl Paranagram {
    pub fn new(path_data: &str) -> io::Result<Self> {
        // Open and read the data file
        let path = Path::new(path_data);
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut words_len = 0;

        // Parse the content of the data file to create a vec of all Word
        let mut sacamot = buffer
            .lines()
            .filter_map(|s| {
                let s = s.trim_end().to_owned();
                if s.len() != 0 {
                    words_len += 1;
                    Some(Word::new(&s[..]))
                } else {
                    None
                }
            })
            .collect::<Vec<Word>>();

        // Return our Paranagram
        Ok(Self {
            path_data: path_data.to_owned(),
            sacamot,
        })
    }

    fn existing_anagrams(&self, sentence: &Word) -> Vec<&Word> {
        self.sacamot.par_iter().filter_map(|word| {
            if word.len() > sentence.len() {
                return None;
            }
            if sentence.contains(word) {
                Some(word)
            } else {
                None
            }
        }).collect()
    }

    pub fn generate_anagrams(&self, sentence: &Word) -> Vec<Vec<&Word>> {
        let anagrams = self.existing_anagrams(sentence);
        let combination = find_sum(anagrams.into_iter(), sentence.weight(), vec![]);
        combination.into_par_iter().filter_map(|c| {
            if &c.iter().fold(HashMap::new(), |mut acc, w| {
                acc.merge(w.letters());
                acc
            }) == sentence.letters() {
                Some(c)
            } else {
                None
            }
        }).collect::<Vec<Vec<&Word>>>()
    }
}

impl fmt::Debug for Paranagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't print the field "sacamot" because it's too large an uninteresting
        f.debug_struct("Paranagram")
            .field("path_data", &self.path_data)
            .field(
                "sacamot_len",
                &self.sacamot.len(),
            )
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore]
    fn init() {
        let start = Instant::now();
        let paranagram = Paranagram::new("data/word.txt");
        println!("{:?} in {:?}", paranagram, start.elapsed());
    }

    #[test]
    #[ignore]
    fn find_all_anagram_of_a_sentence() {
        let word = Word::new("Les parisiennes sont très jolies");
        let paranagram = Paranagram::new("data/word.txt").unwrap();
        let instant = Instant::now();
        let anagrams = paranagram.existing_anagrams(&word);
        assert_eq!(anagrams.len(), 14005);
        println!("{:?}", instant.elapsed());
    }
}
