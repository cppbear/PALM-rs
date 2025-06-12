// Answer 0

#[derive(Default)]
struct HashTable<T> {
    data: std::collections::HashMap<u64, T>,
}

impl<T> HashTable<T> {
    fn insert(&mut self, key: u64, value: T) {
        self.data.insert(key, value);
    }

    unsafe fn get_many_mut_pointers<const N: usize>(
        &mut self,
        hashes: [u64; N],
        mut eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<&mut T>; N] {
        let mut results = [None; N];
        for (i, &hash) in hashes.iter().enumerate() {
            if let Some(value) = self.data.get_mut(&hash) {
                results[i] = Some(value);
            }
        }
        results
    }

    pub fn get_many_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<&mut T>; N] {
        unsafe {
            let ptrs = self.get_many_mut_pointers(hashes, eq);

            for (i, cur) in ptrs.iter().enumerate() {
                if cur.is_some() && ptrs[..i].contains(&cur) {
                    panic!("duplicate keys found");
                }
            }

            ptrs.map(|ptr| ptr.map(|mut ptr| ptr.as_mut()))
        }
    }
}

#[test]
fn test_get_many_mut_unique_keys() {
    let mut table = HashTable::default();
    table.insert(1, 10);
    table.insert(2, 20);
    table.insert(3, 30);
    
    let hashes = [1, 2, 3];
    let results: [Option<&mut i32>; 3] = table.get_many_mut(hashes, |i, k| *k == *(&[10, 20, 30][i]));

    assert_eq!(results[0], Some(&mut 10));
    assert_eq!(results[1], Some(&mut 20));
    assert_eq!(results[2], Some(&mut 30));
}

#[test]
fn test_get_many_mut_duplicate_keys() {
    let mut table = HashTable::default();
    table.insert(1, 10);
    table.insert(2, 20);
    
    let hashes = [1, 2, 1];

    let result = std::panic::catch_unwind(|| {
        table.get_many_mut(hashes, |i, k| *k == *(&[10, 20][i]));
    });
    
    assert!(result.is_err());
}

#[test]
fn test_get_many_mut_non_existent_key() {
    let mut table = HashTable::default();
    table.insert(1, 10);
    
    let hashes = [1, 2, 3];
    let results: [Option<&mut i32>; 3] = table.get_many_mut(hashes, |i, k| *k == *(&[10][i]));

    assert_eq!(results[0], Some(&mut 10));
    assert_eq!(results[1], None);
    assert_eq!(results[2], None);
}

#[test]
fn test_get_many_mut_empty_table() {
    let mut table = HashTable::default();
    let hashes = [1, 2, 3];
    let results: [Option<&mut i32>; 3] = table.get_many_mut(hashes, |i, k| false);

    assert_eq!(results, [None, None, None]);
}

