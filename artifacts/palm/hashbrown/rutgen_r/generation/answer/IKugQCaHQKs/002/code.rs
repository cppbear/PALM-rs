// Answer 0

#[test]
fn test_get_mut_returns_none_when_element_not_found() {
    struct Table {
        data: Vec<Option<u64>>,
    }

    impl Table {
        pub fn new() -> Self {
            Table { data: vec![None; 10] }
        }

        pub fn find(&mut self, hash: u64, eq: impl FnMut(&u64) -> bool) -> Option<&mut Option<u64>> {
            for entry in &mut self.data {
                if let Some(value) = entry {
                    if eq(value) {
                        return Some(entry);
                    }
                }
            }
            None
        }

        pub fn get_mut(&mut self, hash: u64, eq: impl FnMut(&u64) -> bool) -> Option<&mut u64> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { bucket.as_mut() }),
                None => None,
            }
        }
    }

    let mut table = Table::new();
    let result = table.get_mut(42, |&x| x == 10);
    assert_eq!(result, None);
}

