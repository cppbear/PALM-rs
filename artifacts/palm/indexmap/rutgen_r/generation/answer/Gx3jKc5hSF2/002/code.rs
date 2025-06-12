// Answer 0

fn reverse_test() {
    struct TestMap {
        entries: Vec<i32>,
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new(entries: Vec<i32>, indices: Vec<usize>) -> Self {
            Self { entries, indices }
        }

        fn reverse(&mut self) {
            self.entries.reverse();
            let len = self.entries.len();
            for i in &mut self.indices {
                *i = len - *i - 1;
            }
        }
    }

    #[test]
    fn test_reverse_with_empty_entries() {
        let mut map = TestMap::new(Vec::new(), Vec::new());
        map.reverse();
        assert_eq!(map.entries, Vec::new());
        assert_eq!(map.indices, Vec::new());
    }

    #[test]
    fn test_reverse_with_single_entry() {
        let mut map = TestMap::new(vec![42], vec![0]);
        map.reverse();
        assert_eq!(map.entries, vec![42]);
        assert_eq!(map.indices, vec![0]);
    }

    #[test]
    fn test_reverse_with_multiple_entries() {
        let mut map = TestMap::new(vec![1, 2, 3], vec![0, 1, 2]);
        map.reverse();
        assert_eq!(map.entries, vec![3, 2, 1]);
        assert_eq!(map.indices, vec![2, 1, 0]);
    }

    #[test]
    #[should_panic]
    fn test_reverse_with_invalid_indices() {
        let mut map = TestMap::new(vec![1, 2, 3], vec![0, 1, 3]); // index 3 is out of bounds
        map.reverse();
    }
}

