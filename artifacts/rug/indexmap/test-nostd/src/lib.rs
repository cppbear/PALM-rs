#![no_std]

use core::hash::BuildHasherDefault;
use core::hash::Hasher;

use indexmap::IndexMap;
use indexmap::IndexSet;

#[derive(Default)]
struct BadHasher(u64);

impl Hasher for BadHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 += byte as u64
        }
    }
}

type Map<K, V> = IndexMap<K, V, BuildHasherDefault<BadHasher>>;
type Set<T> = IndexSet<T, BuildHasherDefault<BadHasher>>;

pub fn test_compile() {
    let mut map = Map::default();
    map.insert(1, 1);
    map.insert(2, 4);
    for (_, _) in map.iter() {}

    let _map2 = Map::from_iter(Some((1, 1)));

    let mut set = Set::default();
    set.insert("a");
}

#[cfg(test)]
mod tests_llm_16_727 {
    use super::*; // Assuming the `BadHasher` struct is in the same module

use crate::*;
    use core::hash::Hasher;

    #[test]
    fn test_finish() {
        // Create an instance of BadHasher
        let hasher = BadHasher(42);
        
        // Call the finish method
        let result = hasher.finish();
        
        // Assert the expected result
        assert_eq!(result, 42);
    }
}

#[cfg(test)]
mod tests_llm_16_728 {
    use super::*;

use crate::*;
    use core::hash::Hasher;

    #[test]
    fn test_write() {
        let mut hasher = BadHasher(0);
        hasher.write(&[1, 2, 3, 4, 5]);
        assert_eq!(hasher.finish(), 15);
        
        hasher.write(&[10, 20]);
        assert_eq!(hasher.finish(), 45);
        
        hasher.write(&[]);
        assert_eq!(hasher.finish(), 45);
        
        hasher.write(&[255]);
        assert_eq!(hasher.finish(), 300);
    }
}
