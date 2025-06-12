// Answer 0

#[test]
fn test_get_many_unchecked_mut() {
    use hashbrown::{HashTable, Global};
    use std::hash::{Hasher, BuildHasher, Hasher as HasherTrait};

    struct DummyHasher {
        hash: u64,
    }

    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, bytes: &[u8]) {
            self.hash ^= bytes.iter().map(|&b| b as u64).sum::<u64>();
        }
    }

    impl BuildHasher for DummyHasher {
        type Hasher = Self;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher { hash: 0 }
        }
    }

    let mut table: HashTable<(&str, u32), Global> = HashTable::new_in(Global);

    let hasher = |val: &str| {
        let mut hasher = DummyHasher { hash: 0 };
        hasher.write(val.as_bytes());
        hasher.finish()
    };

    let entries = [
        ("Apple", 1),
        ("Banana", 2),
        ("Cherry", 3),
        ("Date", 4),
    ];

    for (k, v) in &entries {
        table.insert_unique(hasher(k), (*k, *v), |&(k, _)| hasher(k));
    }

    let hashes: [u64; 2] = [hasher("Apple"), hasher("Banana")];
    let got = unsafe {
        table.get_many_unchecked_mut(hashes, |i, val| {
            let keys = ["Apple", "Banana"];
            keys[i] == val.0
        })
    };

    assert_eq!(got, [Some(&mut ("Apple", 1)), Some(&mut ("Banana", 2))]);

    let hashes_missing: [u64; 2] = [hasher("Apple"), hasher("NonExistentKey")];
    let got_missing = unsafe {
        table.get_many_unchecked_mut(hashes_missing, |i, val| {
            let keys = ["Apple", "NonExistentKey"];
            keys[i] == val.0
        })
    };

    assert_eq!(got_missing, [Some(&mut ("Apple", 1)), None]);
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_panic() {
    use hashbrown::{HashTable, Global};
    use std::hash::{Hasher, BuildHasher};

    struct DummyHasher {
        hash: u64,
    }

    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, bytes: &[u8]) {
            self.hash ^= bytes.iter().map(|&b| b as u64).sum::<u64>();
        }
    }

    impl BuildHasher for DummyHasher {
        type Hasher = Self;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher { hash: 0 }
        }
    }

    let mut table: HashTable<(&str, u32), Global> = HashTable::new_in(Global);

    let hasher = |val: &str| {
        let mut hasher = DummyHasher { hash: 0 };
        hasher.write(val.as_bytes());
        hasher.finish()
    };

    let entries = [
        ("Key1", 1),
        ("Key2", 2),
    ];

    for (k, v) in &entries {
        table.insert_unique(hasher(k), (*k, *v), |&(k, _)| hasher(k));
    }

    let hashes: [u64; 3] = [hasher("Key1"), hasher("Key2"), hasher("Key1")]; // Duplicate hash to trigger panic
    unsafe {
        table.get_many_unchecked_mut(hashes, |i, val| {
            let keys = ["Key1", "Key2", "Key1"];
            keys[i] == val.0
        });
    };
}

