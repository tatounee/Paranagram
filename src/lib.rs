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
}

impl Paranagram {
    fn new(path_data: &str) -> io::Result<Self> {
        // Open and read the data file
        let path = Path::new(path_data);
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        // Parse the content of the data file to create an vec of all word
            .split("\n")
            })
            .collect();

        // Return our Paranagram
        Ok(Self {
            path_data: path_data.to_owned(),
        })
    }


    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    }

}
