// Answer 0

#[test]
fn test_get_many_unchecked_mut_valid() {
    use hashbrown::HashTable;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct SimpleHasher {
        hash: u64,
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, _: &[u8]) {
            // no-op for simplicity
        }
    }

    let mut table = HashTable::new_in(BuildHasherDefault::<SimpleHasher>::default());

    let entries = [("A", 1), ("B", 2), ("C", 3)];
    for &(key, value) in &entries {
        unsafe {
            let hash = key.len() as u64; // simplistic hash for the demonstration
            table.insert_unique(hash, (key, value), |&k| hash); // insert unique elements
        }
    }

    let hashes = [1, 2]; // simplistic hashes
    let got = unsafe {
        table.get_many_unchecked_mut(hashes, |i, val| {
            let keys = ["A", "B"];
            keys[i] == val.0
        })
    };

    assert_eq!(got, [
        Some(&mut ("A", 1)),
        Some(&mut ("B", 2))
    ]);
}

#[test]
fn test_get_many_unchecked_mut_some_missing() {
    use hashbrown::HashTable;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct SimpleHasher {
        hash: u64,
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, _: &[u8]) {
            // no-op for simplicity
        }
    }

    let mut table = HashTable::new_in(BuildHasherDefault::<SimpleHasher>::default());

    let entries = [("A", 1), ("B", 2)];
    for &(key, value) in &entries {
        unsafe {
            let hash = key.len() as u64;
            table.insert_unique(hash, (key, value), |&k| hash);
        }
    }

    let hashes = [1, 10]; // 10 does not exist
    let got = unsafe {
        table.get_many_unchecked_mut(hashes, |i, val| {
            let keys = ["A", "B"];
            keys[i] == val.0
        })
    };

    assert_eq!(got, [
        Some(&mut ("A", 1)),
        None
    ]);
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_overlapping_keys() {
    use hashbrown::HashTable;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct SimpleHasher {
        hash: u64,
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, _: &[u8]) {
            // no-op for simplicity
        }
    }

    let mut table = HashTable::new_in(BuildHasherDefault::<SimpleHasher>::default());

    let entries = [("X", 10), ("Y", 20)];
    for &(key, value) in &entries {
        unsafe {
            let hash = key.len() as u64;
            table.insert_unique(hash, (key, value), |&k| hash);
        }
    }

    let hashes = [1, 1]; // both hashes refer to the same location
    unsafe {
        let _ = table.get_many_unchecked_mut(hashes, |i, val| {
            let keys = ["X", "Y"];
            keys[i] == val.0
        }); // this should panic due to overlapping keys
    }
}

