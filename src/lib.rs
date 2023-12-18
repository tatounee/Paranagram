#![allow(unused_imports)]

mod utils;
use utils::*;

pub mod word;
use word::ToIndexAndWeight;
pub use word::Word;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use rayon::prelude::*;

use crate::word::FromIndexAndWeight;

pub struct Paranagram {
    path_data: String,
    sacamot: Vec<Word>,
}

impl Paranagram {
    pub fn new(path: &Path) -> io::Result<Self> {
        // Open and read the data file
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut words_len = 0;

        // Parse the content of the data file to create a vec of all Word
        let sacamot = buffer
            .lines()
            .filter_map(|s| {
                let s = s.trim_end().to_owned();
                if !s.is_empty() {
                    words_len += 1;
                    Some(Word::new(&s))
                } else {
                    None
                }
            })
            .collect::<Vec<Word>>();

        // Return our Paranagram
        Ok(Self {
            path_data: path.to_str().unwrap().to_owned(),
            sacamot,
        })
    }

    pub fn existing_anagrams(&self, sentence: &Word) -> Vec<&Word> {
        self.sacamot
            .iter()
            .filter(|word| {
                if word.len() > sentence.len() {
                    return false;
                }
                sentence.contains(word)
            })
            .collect()
    }

    pub fn generate_anagrams(&self, sentence: &str) -> Vec<Vec<&Word>> {
        let sentence = Word::new(sentence);
        let anagrams = self.existing_anagrams(&sentence);
        let tuple_anagrams = anagrams.to_index_and_weight();

        find_sum(&tuple_anagrams, sentence.weight())
            .into_iter()
            .map(|x| (anagrams.from_index_and_weight(x)))
            .filter_map(|c| {
                let letters = &c.iter().fold(HashMap::new(), |mut acc, w| {
                    acc.merge(w.letters());
                    acc
                });
                if letters == sentence.letters() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<&Word>>>()
    }

    pub fn generate_anagrams_debug(
        &self,
        sentence: &str,
        current_step: usize,
        goal: usize,
    ) -> Vec<Vec<&Word>> {
        let sentence = Word::new(sentence);
        let anagrams = self.existing_anagrams(&sentence);

        println!("[{}/{}] Possible anagrams found", current_step + 1, goal);

        let tuple_anagrams = anagrams.to_index_and_weight();
        let combination = find_sum(&tuple_anagrams, sentence.weight());
        let combination = combination
            .into_iter()
            .map(|x| anagrams.from_index_and_weight(x))
            .collect::<Vec<Vec<&Word>>>();

        println!("[{}/{}] Possible sentences found", current_step + 2, goal);

        let out = combination
            .into_par_iter()
            .filter_map(|c| {
                let letters = &c.iter().fold(HashMap::new(), |mut acc, w| {
                    acc.merge(w.letters());
                    acc
                });
                if letters == sentence.letters() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<&Word>>>();

        println!("[{}/{}]  All sentences found", current_step + 3, goal);

        out
    }
}

impl fmt::Debug for Paranagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't print the field "sacamot" because it's too large an uninteresting
        f.debug_struct("Paranagram")
            .field("path_data", &self.path_data)
            .field("sacamot_len", &self.sacamot.len())
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
        let paranagram = Paranagram::new(Path::new("data/word.txt"));
        println!("{:?} in {:?}", paranagram, start.elapsed());
        println!("{:?}", paranagram.unwrap().sacamot[0]);
    }

    #[test]
    #[ignore]
    fn find_all_anagram_of_a_sentence() {
        let word = Word::new("Les parisiennes sont très jolies");
        let paranagram = Paranagram::new(Path::new("data/word.txt")).unwrap();
        let instant = Instant::now();
        let anagrams = paranagram.existing_anagrams(&word);
        assert_eq!(anagrams.len(), 14005);
        println!("{:?}", instant.elapsed());
    }

    #[test]
    fn find_all_anagramed_sentence_of_a_sentence() {
        let start = Instant::now();
        let paranagram = Paranagram::new(Path::new("data/word.txt")).unwrap();
        let middle = Instant::now();
        let anagrams = paranagram.generate_anagrams("parisiennes");
        let end = Instant::now();

        let mut buffer = String::new();
        anagrams.iter().for_each(|v| {
            for w in v.iter() {
                buffer.push_str(&format!("{} ", w))
            }
            buffer.push('\n');
        });
        buffer.push_str(&format!("len: {}\n", anagrams.len()));
        buffer.push_str(&format!(
            "[{:?}] {:?} + {:?}",
            end - start,
            middle - start,
            end - middle
        ));

        let path = Path::new("paranagram.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(buffer.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
