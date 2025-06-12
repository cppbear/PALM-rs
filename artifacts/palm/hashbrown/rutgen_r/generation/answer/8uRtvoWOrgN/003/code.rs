// Answer 0

#[test]
fn test_get_many_mut_distinct_keys() {
    struct Table {
        data: std::collections::HashMap<u64, i32>,
    }

    impl Table {
        fn new() -> Self {
            Table {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<&'_ mut i32>; N] {
            // This mimics the functionality of the original function without external dependencies
            let mut results = [None; N];
            let mut seen = std::collections::HashSet::new();

            for (i, hash) in hashes.iter().enumerate() {
                if let Some(v) = self.data.get_mut(hash) {
                    if seen.insert(v as *mut i32) {
                        results[i] = Some(v);
                    }
                }
            }
            results
        }
    }

    let mut table = Table::new();
    table.data.insert(1, 10);
    table.data.insert(2, 20);
    table.data.insert(3, 30);

    let results = table.get_many_mut([1, 2, 3], |i, k| *k == i as i32 + 10);
    assert_eq!(results, [Some(&mut 10), Some(&mut 20), Some(&mut 30)]);
}

#[test]
fn test_get_many_mut_duplicate_keys() {
    struct Table {
        data: std::collections::HashMap<u64, i32>,
    }

    impl Table {
        fn new() -> Self {
            Table {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<&'_ mut i32>; N] {
            let mut results = [None; N];
            let mut seen = std::collections::HashSet::new();

            for (i, hash) in hashes.iter().enumerate() {
                if let Some(v) = self.data.get_mut(hash) {
                    if seen.insert(v as *mut i32) {
                        results[i] = Some(v);
                    }
                }
            }
            results
        }
    }

    let mut table = Table::new();
    table.data.insert(1, 10);
    table.data.insert(2, 20);
    table.data.insert(3, 30);

    let result = std::panic::catch_unwind(|| {
        table.get_many_mut([1, 2, 1], |i, k| *k == i as i32 + 10);
    });

    assert!(result.is_err());
}

#[test]
fn test_get_many_mut_key_not_found() {
    struct Table {
        data: std::collections::HashMap<u64, i32>,
    }

    impl Table {
        fn new() -> Self {
            Table {
                data: std::collections::HashMap::new(),
            }
        }

        fn get_many_mut<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<&'_ mut i32>; N] {
            let mut results = [None; N];

            for (i, hash) in hashes.iter().enumerate() {
                results[i] = self.data.get_mut(hash);
            }
            results
        }
    }

    let mut table = Table::new();
    table.data.insert(1, 10);
    table.data.insert(2, 20);
    
    let results = table.get_many_mut([3, 4], |i, k| *k == i as i32 + 10);
    assert_eq!(results, [None, None]);
}

