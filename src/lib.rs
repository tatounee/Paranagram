#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str;

use itertools::Itertools;
use unidecode::unidecode;

const NBR_LETTER: usize = 26;
#[derive(Debug)]
struct Paranagram {
    path_data: String,
    sacamot: Vec<Vec<String>>,
}

impl Paranagram {
    fn new(path_data: &str) -> io::Result<Self> {
        // Open and read the data file
        let path = Path::new(path_data);
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        // Parse the content of the data file to create an vec of all word
        let mut buffer: Vec<&str> = buffer
            .split("\n")
            .map(|x| {
                let row = x.as_bytes();
                str::from_utf8(&row[0..row.len() - 1]).unwrap()
            })
            .collect();
        
        // Patition datas by alphabetical order
        let mut sacamot = vec![];
        let mut tmp = [0; 4];
        for i in 97..=122 {
            let letter = (i as u8 as char).to_string();
            let (group, new_buffer) = buffer.iter().partition(|x| {
                unidecode(&x.chars().next().unwrap().encode_utf8(&mut tmp)) == letter
            });
            buffer = new_buffer;
            sacamot.push(group);
        }

        // Return our Paranagram
        Ok(Self {
            path_data: path_data.to_owned(),
            sacamot: sacamot
                .iter()
                // Convert all &str to String 
                .map(|x| x.iter().map(|&s| s.to_owned()).collect()) 
                .collect(),
        })
    }

    fn find_anagram(&self, sentence: &str) -> Vec<Vec<&str>> {
        let mut letters = sentence
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<Vec<char>>();

        let mut buf = [0; 4];
        let mut existing_letters = letters
            .clone()
            .iter()
            .map(|c| unidecode(c.encode_utf8(&mut buf)).chars().next().unwrap())
            .collect::<Vec<char>>();
        existing_letters.sort_unstable();
        let existing_letters = existing_letters.into_iter().dedup().collect::<Vec<char>>();

        for i in existing_letters.into_iter() {
            let mut buf = [0; 4];
            println!(
                "{} as u8 = {},  unidecode = {}",
                i,
                i as u8,
                unidecode(i.encode_utf8(&mut buf))
            );
        }
        vec![vec![]]
    }

    fn debug_(&self) {
        self.sacamot.iter().for_each(|g| {
            println!("{}", format!("{} | {}", g[0], g[1]));
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_paranagram() {
        let para = Paranagram::new("data/word.txt");
        match para {
            Err(e) => println!("{}", e),
            Ok(p) => {
                p.debug_();
                println!("len = {}", p.sacamot.len());
                p.sacamot
                    .iter()
                    .enumerate()
                    .for_each(|(i, v)| println!("{} -> {}", (i as u8 + 97) as char, v.len()));
            }
        }
    }

    #[test]
    fn test_find_anagram() {
        let para = Paranagram::new("data/word.txt").unwrap();
        para.find_anagram("je t'aime beaucoup éà");
    }
}
