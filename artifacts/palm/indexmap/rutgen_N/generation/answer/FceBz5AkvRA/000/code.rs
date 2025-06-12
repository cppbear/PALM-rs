// Answer 0

#[test]
fn test_binary_search_by_found() {
    struct SortedSet {
        data: Vec<i32>,
    }

    impl SortedSet {
        fn new(data: Vec<i32>) -> Self {
            Self { data: data.into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect() }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> std::cmp::Ordering,
        {
            self.as_slice().binary_search_by(f)
        }
    }

    let set = SortedSet::new(vec![1, 3, 5, 7, 9]);
    let result = set.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_not_found() {
    struct SortedSet {
        data: Vec<i32>,
    }

    impl SortedSet {
        fn new(data: Vec<i32>) -> Self {
            Self { data: data.into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect() }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> std::cmp::Ordering,
        {
            self.as_slice().binary_search_by(f)
        }
    }

    let set = SortedSet::new(vec![1, 3, 5, 7, 9]);
    let result = set.binary_search_by(|&x| x.cmp(&4));
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_by_insert_position_for_greatest() {
    struct SortedSet {
        data: Vec<i32>,
    }

    impl SortedSet {
        fn new(data: Vec<i32>) -> Self {
            Self { data: data.into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect() }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data
        }

        fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> std::cmp::Ordering,
        {
            self.as_slice().binary_search_by(f)
        }
    }

    let set = SortedSet::new(vec![1, 3, 5, 7, 9]);
    let result = set.binary_search_by(|&x| x.cmp(&10));
    assert_eq!(result, Err(5));
}

