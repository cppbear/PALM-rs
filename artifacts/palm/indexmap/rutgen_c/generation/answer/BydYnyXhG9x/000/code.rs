// Answer 0

#[test]
fn test_retain_removes_elements() {
    struct TestMap {
        data: Vec<u32>,
    }

    impl TestMap {
        fn retain<F>(&mut self, keep: F)
        where
            F: FnMut(&u32) -> bool,
        {
            self.data.retain(keep);
        }
    }

    let mut map = TestMap { data: vec![1, 2, 3, 4, 5] };
    map.retain(|&x| x % 2 == 0);
    assert_eq!(map.data, vec![2, 4]);
}

#[test]
fn test_retain_keeps_all_elements() {
    struct TestMap {
        data: Vec<u32>,
    }

    impl TestMap {
        fn retain<F>(&mut self, keep: F)
        where
            F: FnMut(&u32) -> bool,
        {
            self.data.retain(keep);
        }
    }

    let mut map = TestMap { data: vec![1, 2, 3] };
    map.retain(|_| true);
    assert_eq!(map.data, vec![1, 2, 3]);
}

#[test]
fn test_retain_removes_all_elements() {
    struct TestMap {
        data: Vec<u32>,
    }

    impl TestMap {
        fn retain<F>(&mut self, keep: F)
        where
            F: FnMut(&u32) -> bool,
        {
            self.data.retain(keep);
        }
    }

    let mut map = TestMap { data: vec![1, 2, 3] };
    map.retain(|_| false);
    assert_eq!(map.data, vec![]);
}

#[test]
fn test_retain_with_empty_set() {
    struct TestMap {
        data: Vec<u32>,
    }

    impl TestMap {
        fn retain<F>(&mut self, keep: F)
        where
            F: FnMut(&u32) -> bool,
        {
            self.data.retain(keep);
        }
    }

    let mut map = TestMap { data: vec![] };
    map.retain(|_| true);
    assert_eq!(map.data, vec![]);
}

