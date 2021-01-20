use std::collections::HashMap;

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
    fn str_with_len_of_100() {
        let start = Instant::now();
        "abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij".to_hashmap();
        // println!("str_with_len_of_100 : {:?}", start.elapsed());
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
}
