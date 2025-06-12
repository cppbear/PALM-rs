// Answer 0

#[test]
fn test_get_many_mut_unique_hashes() {
    struct HashMap {
        entries: Vec<(u64, i32)>,
    }

    impl HashMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn insert(&mut self, hash: u64, value: i32) {
            self.entries.push((hash, value));
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<&'_ mut i32>; N] {
            let mut results = [None; N];
            let mut pointers: Vec<*mut i32> = Vec::new();

            for (i, &hash) in hashes.iter().enumerate() {
                if let Some(pos) = self.entries.iter().position(|(h, _)| *h == hash) {
                    let value = &mut self.entries[pos].1;
                    pointers.push(value);
                    results[i] = Some(value);
                }
            }

            for (i, ptr) in pointers.iter().enumerate() {
                if ptr.is_some() && pointers[..i].contains(ptr) {
                    panic!("duplicate keys found");
                }
            }

            results
        }
    }

    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);

    let hashes = [1, 2, 3];
    let result = map.get_many_mut(hashes, |i, val| *val == (i as i32 + 10));
    
    assert_eq!(result, [Some(&mut 10), Some(&mut 20), Some(&mut 30)]);
}

#[test]
#[should_panic]
fn test_get_many_mut_duplicate_hashes() {
    struct HashMap {
        entries: Vec<(u64, i32)>,
    }

    impl HashMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn insert(&mut self, hash: u64, value: i32) {
            self.entries.push((hash, value));
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<&'_ mut i32>; N] {
            let mut results = [None; N];
            let mut pointers: Vec<*mut i32> = Vec::new();

            for (i, &hash) in hashes.iter().enumerate() {
                if let Some(pos) = self.entries.iter().position(|(h, _)| *h == hash) {
                    let value = &mut self.entries[pos].1;
                    pointers.push(value);
                    results[i] = Some(value);
                }
            }

            for (i, ptr) in pointers.iter().enumerate() {
                if ptr.is_some() && pointers[..i].contains(ptr) {
                    panic!("duplicate keys found");
                }
            }

            results
        }
    }

    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(1, 30); // Duplicate hash

    let hashes = [1, 1];
    let _ = map.get_many_mut(hashes, |i, val| *val == (i as i32 + 10)); // This should panic due to duplicates.
}

