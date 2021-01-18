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
    sacamot: Vec<(usize, Vec<Word>)>,
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
        let mut words = buffer
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

        let mut sacamot = vec![];

        let mut len = 1;
        loop {
            if words_len == 0 {
                break;
            }
            let (same_len, new_words) = words.into_iter().partition(|x| x.word.len() == len);
            words = new_words;

            words_len -= same_len.len();
            sacamot.push((len, same_len));

            len += 1;
        }

        // Return our Paranagram
        Ok(Self {
            path_data: path_data.to_owned(),
            sacamot,
        })
    }

    pub fn existing_anagrams(&self, sentence: &str) -> Vec<String> {
        let sentence = Word::new(sentence);
        let mut anagrams = Vec::new();
        for (len, sac) in self.sacamot.iter() {
            if len > &sentence.len() {
                continue;
            }
            for w in sac.iter() {
                if sentence.contains(w) {
                    anagrams.push(w.word.to_owned())
                }
            }
        }
        anagrams
    }
}

impl fmt::Debug for Paranagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't print the field "sacamot" because it's too large an uninteresting
        f.debug_struct("Paranagram")
            .field("path_data", &self.path_data)
            .field(
                "sacamot_len",
                &self.sacamot.iter().map(|x| x.1.len()).sum::<usize>(),
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
    fn find_all_anagram_of_a_word() {
        let word = "Les parisiennes sont trés jolies";
        let paranagram = Paranagram::new("data/word.txt").unwrap();
        let instant = Instant::now();
        let anagrams = paranagram.existing_anagrams(word);
        println!("{:?}", anagrams);
        println!("{:?}", instant.elapsed());
    }
}
