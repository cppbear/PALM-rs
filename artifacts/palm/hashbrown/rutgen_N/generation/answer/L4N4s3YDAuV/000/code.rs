// Answer 0

#[test]
fn test_get_existing_element() {
    struct TestTable<T> {
        elements: Vec<(u64, T)>,
    }

    impl<T> TestTable<T> {
        fn new() -> Self {
            Self { elements: Vec::new() }
        }

        fn insert(&mut self, hash: u64, value: T) {
            self.elements.push((hash, value));
        }

        fn find<F>(&self, hash: u64, mut eq: F) -> Option<&(u64, T)>
        where
            F: FnMut(&T) -> bool,
        {
            self.elements.iter().find(|&&(_, ref v)| eq(v) && hash == hash)
        }

        pub fn get(&self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { &bucket.1 }),
                None => None,
            }
        }
    }

    let mut table = TestTable::new();
    table.insert(1, "value1");
    table.insert(2, "value2");

    let result = table.get(1, |&v| v == "value1");
    assert_eq!(result, Some(&"value1"));
}

#[test]
fn test_get_non_existing_element() {
    struct TestTable<T> {
        elements: Vec<(u64, T)>,
    }

    impl<T> TestTable<T> {
        fn new() -> Self {
            Self { elements: Vec::new() }
        }

        fn insert(&mut self, hash: u64, value: T) {
            self.elements.push((hash, value));
        }

        fn find<F>(&self, hash: u64, mut eq: F) -> Option<&(u64, T)>
        where
            F: FnMut(&T) -> bool,
        {
            self.elements.iter().find(|&&(_, ref v)| eq(v) && hash == hash)
        }

        pub fn get(&self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { &bucket.1 }),
                None => None,
            }
        }
    }

    let mut table = TestTable::new();
    table.insert(1, "value1");
    table.insert(2, "value2");

    let result = table.get(3, |&v| v == "value3");
    assert_eq!(result, None);
}

