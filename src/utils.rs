use crate::word::{self, Word};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::thread;

pub(crate) trait IntoHashMap<K, V> {
    fn to_hashmap(self) -> HashMap<K, V>;
}

impl IntoHashMap<char, u16> for &str {
    fn to_hashmap(self) -> HashMap<char, u16> {
        let mut letters: HashMap<char, u16> = HashMap::new();
        self.chars().for_each(|c| {
            let mut lettre_counter = letters.entry(c).or_insert(0);
            *lettre_counter += 1;
        });
        letters
    }
}

pub(crate) trait HashMapUtils<K, V> {
    fn contains(&self, other: &HashMap<K, V>) -> bool;
    fn merge(&mut self, other: &HashMap<K, V>);
}

impl HashMapUtils<char, u16> for HashMap<char, u16> {
    fn contains(&self, other: &HashMap<char, u16>) -> bool {
        for (key, val) in other.iter() {
            if self.contains_key(key) == false {
                return false;
            }
            if self.get(key).unwrap() < val {
                return false;
            }
        }
        true
    }

    fn merge(&mut self, other: &HashMap<char, u16>) {
        for (key, val) in other.iter() {
            let entry = self.entry(*key).or_insert(0);
            *entry += val
        }
    }
}

pub trait ToTupleIndex {
    fn to_tuple_index(&self) -> Vec<(usize, usize)>;
}

pub trait FromTupleIndex {
    fn from_tuple_index(&self, tuple: Vec<(usize, usize)>) -> Self;
}

impl ToTupleIndex for Vec<&Word> {
    #[inline]
    fn to_tuple_index(&self) -> Vec<(usize, usize)> {
        self.iter()
            .enumerate()
            .map(|(i, w)| (i, w.weight()))
            .collect()
    }
}

impl FromTupleIndex for Vec<&Word> {
    #[inline]
    fn from_tuple_index(&self, tuple: Vec<(usize, usize)>) -> Self {
        tuple.iter().map(|x| *self.get(x.0).unwrap()).collect()
    }
}

pub trait Extract {
    fn extract(&self) -> usize;
}

impl Extract for (usize, usize) {
    fn extract(&self) -> usize {
        self.1
    }
}

// TODO: Add multitheading for this part
#[inline]
pub fn find_sum(mut data: Vec<(usize, usize)>, goal: usize) -> Vec<Vec<(usize, usize)>> {
    data.sort_unstable_by(|a, b| a.extract().cmp(&b.extract()));
    let data1 = data.into_iter().enumerate().rev();
    {
        let mut data = data1;
        let goal = goal;
        let rest = 0;
        let floor: Vec<(usize, usize)> = vec![];
        let mut buffer = vec![];
        let floor_sum = floor.iter().map(|x| x.extract()).sum::<usize>();
        let mut thread_vec = Vec::new();
        while let Some((index, number)) = data.next() {
            println!("{}", index);
            match (number.extract() + floor_sum).cmp(&goal) {
                Ordering::Equal => {
                    let mut v = vec![number];
                    v.extend_from_slice(&floor);
                    buffer.push(v)
                }
                Ordering::Less => {
                    if (index + 1) * number.extract() < rest {
                        break;
                    }
                    let mut v = vec![number];
                    let cloned_data = data.clone();
                    v.extend_from_slice(&floor);
                    let th = thread::spawn(move || {
                        find_sum_rec(
                            cloned_data,
                            goal,
                            goal - floor_sum - number.extract(),
                            v,
                        )
                    });
                    thread_vec.push(th);
                }
            
                Ordering::Greater => {}
            }
        }
        for t in thread_vec {
            buffer.append(&mut t.join().unwrap())
        }
        buffer
    }
}

pub fn find_sum_rec<I>(
    mut data: I,
    goal: usize,
    rest: usize,
    floor: Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>>
where
    I: Iterator<Item = (usize, (usize, usize))> + Clone + Send + Sync,
{
    let mut buffer = vec![];
    let floor_sum = floor.iter().map(|x| x.extract()).sum::<usize>();

   // let mut thread_vec = Vec::new();

    while let Some((index, number)) = data.next() {
        match (number.extract() + floor_sum).cmp(&goal) {
            Ordering::Equal => {
                let mut v = vec![number];
                v.extend_from_slice(&floor);
                buffer.push(v)
            }
            Ordering::Less => {
                if (index + 1) * number.extract() < rest {
                    break;
                }
                let mut v = vec![number];
                v.extend_from_slice(&floor);
                find_sum_rec(
                    data.clone(),
                    goal,
                    goal - floor_sum - number.extract(),
                    v,
                ).into_iter().for_each(|x| buffer.push(x))
            }

            Ordering::Greater => {}
        }
    }
    buffer
}



#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
}

#[macro_export]
macro_rules! vec_word_weight {
    ($($w:expr),* ) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push(Word::new_perso(String::new(), 0, $w, HashMap::new()));
            )*
            vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn empty_str() {
        assert_eq!("".to_hashmap(), HashMap::new())
    }

    #[test]
    fn not_empty_str() {
        let letters = hashmap!['a' => 2, 'b' => 2, 'e' => 1, 'r' => 1];
        assert_eq!("babare".to_hashmap(), letters)
    }

    #[test]
    fn str_with_all_letter() {
        let start = Instant::now();
        "abcdefghijklmnopqrstuvwxuyz".to_hashmap();
        // println!("str_with_all_letter {:?}", start.elapsed());
    }
    #[test]
    fn str_with_all_letter_multiple_time() {
        let start = Instant::now();
        "abcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyz".to_hashmap();
        // println!("str_with_all_letter_multiple_time {:?}", start.elapsed());
    }

    #[test]
    fn test_hashmap_contains_hashmap() {
        let hm1 = hashmap!['a' => 2, 'b' => 1];
        let hm2 = hashmap!['a' => 3, 'b' => 1, 'c' => 1];
        assert!(hm2.contains(&hm1));
        assert!(!hm1.contains(&hm2));
        assert!(hm1.contains(&hm1));
    }

    #[test]
    fn little_data_tuple() {
        // let start = Instant::now();
        let data: Vec<(usize, usize)> = vec![
            (100, 10),
            (50, 5),
            (30, 3),
            (70, 7),
            (50, 5),
            (20, 2),
            (10, 1),
        ]; // (10) (7, 3) (7, 2, 1) (5, 5) (5, 3, 2)
        // let x = find_sum_tuple(data.into_iter(), 10, vec![]);
        // println!("{:?}", x);
        // println!("{:?}", start.elapsed());
    }

    #[test]
    fn merge_existing_key() {
        let mut hm1 = hashmap!['a' => 2, 'b' => 1];
        let hm2 = hashmap!['a' => 3, 'b' => 1];
        hm1.merge(&hm2);
        assert_eq!(hm1, hashmap!['a' => 5, 'b' => 2])
    }

    #[test]
    fn merge_with_new_key() {
        let mut hm1 = hashmap!['a' => 2, 'b' => 1];
        let hm2 = hashmap!['a' => 3, 'b' => 1, 'c' => 2];
        hm1.merge(&hm2);
        assert_eq!(hm1, hashmap!['a' => 5, 'b' => 2, 'c' => 2])
    }
}
