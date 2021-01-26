#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod utils;
mod word;

use utils::*;
use word::Word;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use rayon::prelude::*;

use std::time::Instant;

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
        let instant = Instant::now();
        let anagrams = self.existing_anagrams(sentence);
        println!("0 - {:?}", instant.elapsed());
        let instant = Instant::now();

        let combination = find_sum(anagrams.into_iter(), sentence.weight(), vec![]);
        println!("1 - {:?}", instant.elapsed());
        let instant = Instant::now();

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

    #[test]
    #[ignore]
    fn init() {
        let start = Instant::now();
        let paranagram = Paranagram::new("data/word.txt");
        println!("{:?} in {:?}", paranagram, start.elapsed());
        println!("{:?}", paranagram.unwrap().sacamot[0]);
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

    #[test]
    fn find_all_anagramed_sentence_of_a_sentence() {
        let word = Word::new("les parisiennes");
        let start = Instant::now();
        let paranagram = Paranagram::new("data/word.txt").unwrap();
        let middle = Instant::now();
        let anagrams = paranagram.generate_anagrams(&word);
        let end = Instant::now();

        let mut buffer = String::new();
        anagrams.iter().for_each(|v| {
            for w in v.iter() {
                buffer.push_str(&format!("{} ", w))
            }
            buffer.push('\n');
        });
        buffer.push_str(&format!("len: {}", anagrams.len()));
        buffer.push_str(&format!(
            "[{:?}] {:?} + {:?}",
            end - start,
            middle - start,
            end - middle
        ));

        let path = Path::new("paranagram.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
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
