// Answer 0

#[test]
fn binary_search_by_key_found() {
    struct TestMap {
        entries: Vec<Bucket<&'static str, usize>>,
    }

    impl TestMap {
        fn new(entries: Vec<Bucket<&'static str, usize>>) -> Self {
            Self { entries }
        }

        fn binary_search_by_key<F>(&self, b: &usize, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&&str, &usize) -> usize,
        {
            let len = self.entries.len();
            let mut low = 0;
            let mut high = len;

            while low < high {
                let mid = (low + high) / 2;
                if f(&&self.entries[mid].key, &self.entries[mid].value) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            Ok(low)
        }
    }

    let entries = vec![
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 0, key: "b", value: 2 },
        Bucket { hash: 0, key: "c", value: 3 },
    ];

    let map = TestMap::new(entries);
    let result = map.binary_search_by_key(&2, |key, value| *value);
    assert_eq!(result, Ok(1));
}

#[test]
fn binary_search_by_key_not_found() {
    struct TestMap {
        entries: Vec<Bucket<&'static str, usize>>,
    }

    impl TestMap {
        fn new(entries: Vec<Bucket<&'static str, usize>>) -> Self {
            Self { entries }
        }

        fn binary_search_by_key<F>(&self, b: &usize, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&&str, &usize) -> usize,
        {
            let len = self.entries.len();
            let mut low = 0;
            let mut high = len;

            while low < high {
                let mid = (low + high) / 2;
                if f(&&self.entries[mid].key, &self.entries[mid].value) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            Ok(low)
        }
    }

    let entries = vec![
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 0, key: "b", value: 2 },
        Bucket { hash: 0, key: "c", value: 3 },
    ];

    let map = TestMap::new(entries);
    let result = map.binary_search_by_key(&4, |key, value| *value);
    assert_eq!(result, Ok(3));
}

#[test]
fn binary_search_by_key_empty() {
    struct TestMap {
        entries: Vec<Bucket<&'static str, usize>>,
    }

    impl TestMap {
        fn new(entries: Vec<Bucket<&'static str, usize>>) -> Self {
            Self { entries }
        }

        fn binary_search_by_key<F>(&self, b: &usize, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&&str, &usize) -> usize,
        {
            let len = self.entries.len();
            let mut low = 0;
            let mut high = len;

            while low < high {
                let mid = (low + high) / 2;
                if f(&&self.entries[mid].key, &self.entries[mid].value) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            Ok(low)
        }
    }

    let entries: Vec<Bucket<&'static str, usize>> = vec![];
    let map = TestMap::new(entries);
    let result = map.binary_search_by_key(&1, |key, value| *value);
    assert_eq!(result, Ok(0));
}

