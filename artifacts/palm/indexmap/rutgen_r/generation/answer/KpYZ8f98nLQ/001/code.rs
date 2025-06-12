// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct MyMap {
        data: Vec<(i32, i32)>,
    }

    impl MyMap {
        fn new(data: Vec<(i32, i32)>) -> Self {
            MyMap { data }
        }

        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&(i32, i32)) -> B,
            B: Ord,
        {
            self.data.binary_search_by(|&(k, v)| f(&(k, v)).cmp(b))
        }
    }

    let map = MyMap::new(vec![(1, 10), (2, 20), (3, 30)]);
    let result = map.binary_search_by_key(&20, |&(_, v)| v);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct MyMap {
        data: Vec<(i32, i32)>,
    }

    impl MyMap {
        fn new(data: Vec<(i32, i32)>) -> Self {
            MyMap { data }
        }

        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&(i32, i32)) -> B,
            B: Ord,
        {
            self.data.binary_search_by(|&(k, v)| f(&(k, v)).cmp(b))
        }
    }

    let map = MyMap::new(vec![(1, 10), (2, 20), (3, 30)]);
    let result = map.binary_search_by_key(&25, |&(_, v)| v);
    assert_eq!(result, Err(2));  // Position where it can be inserted
}

#[test]
fn test_binary_search_by_key_boundary_low() {
    struct MyMap {
        data: Vec<(i32, i32)>,
    }

    impl MyMap {
        fn new(data: Vec<(i32, i32)>) -> Self {
            MyMap { data }
        }

        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&(i32, i32)) -> B,
            B: Ord,
        {
            self.data.binary_search_by(|&(k, v)| f(&(k, v)).cmp(b))
        }
    }

    let map = MyMap::new(vec![(1, 10), (2, 20), (3, 30)]);
    let result = map.binary_search_by_key(&5, |&(_, v)| v);
    assert_eq!(result, Err(0));  // Position where it can be inserted at the start
}

#[test]
fn test_binary_search_by_key_boundary_high() {
    struct MyMap {
        data: Vec<(i32, i32)>,
    }

    impl MyMap {
        fn new(data: Vec<(i32, i32)>) -> Self {
            MyMap { data }
        }

        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&(i32, i32)) -> B,
            B: Ord,
        {
            self.data.binary_search_by(|&(k, v)| f(&(k, v)).cmp(b))
        }
    }

    let map = MyMap::new(vec![(1, 10), (2, 20), (3, 30)]);
    let result = map.binary_search_by_key(&35, |&(_, v)| v);
    assert_eq!(result, Err(3));  // Position where it can be inserted at the end
}

