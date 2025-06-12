// Answer 0

#[test]
fn test_partition_point_empty() {
    struct TestSet {
        entries: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { entries: Vec::new() }
        }
        
        fn as_slice(&self) -> &[i32] {
            &self.entries
        }

        fn partition_point<P>(&self, pred: P) -> usize
        where
            P: FnMut(&i32) -> bool,
        {
            self.as_slice().iter().position(|x| !pred(x)).unwrap_or(self.entries.len())
        }
    }

    let set = TestSet::new();
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_all_less() {
    struct TestSet {
        entries: Vec<i32>,
    }

    impl TestSet {
        fn new(entries: Vec<i32>) -> Self {
            TestSet { entries }
        }

        fn as_slice(&self) -> &[i32] {
            &self.entries
        }

        fn partition_point<P>(&self, pred: P) -> usize
        where
            P: FnMut(&i32) -> bool,
        {
            self.as_slice().iter().position(|x| !pred(x)).unwrap_or(self.entries.len())
        }
    }

    let set = TestSet::new(vec![1, 2, 3, 4]);
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 4);
}

#[test]
fn test_partition_point_all_greater() {
    struct TestSet {
        entries: Vec<i32>,
    }

    impl TestSet {
        fn new(entries: Vec<i32>) -> Self {
            TestSet { entries }
        }

        fn as_slice(&self) -> &[i32] {
            &self.entries
        }

        fn partition_point<P>(&self, pred: P) -> usize
        where
            P: FnMut(&i32) -> bool,
        {
            self.as_slice().iter().position(|x| !pred(x)).unwrap_or(self.entries.len())
        }
    }

    let set = TestSet::new(vec![6, 7, 8, 9]);
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_mixed() {
    struct TestSet {
        entries: Vec<i32>,
    }

    impl TestSet {
        fn new(entries: Vec<i32>) -> Self {
            TestSet { entries }
        }

        fn as_slice(&self) -> &[i32] {
            &self.entries
        }

        fn partition_point<P>(&self, pred: P) -> usize
        where
            P: FnMut(&i32) -> bool,
        {
            self.as_slice().iter().position(|x| !pred(x)).unwrap_or(self.entries.len())
        }
    }

    let set = TestSet::new(vec![1, 2, 3, 6, 7, 8]);
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 3);
}

