// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct Set {
        elements: Vec<i32>,
    }

    impl Set {
        fn new(elements: Vec<i32>) -> Self {
            Set { elements }
        }

        fn as_slice(&self) -> &[i32] {
            &self.elements
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = Set::new(vec![1, 3, 5, 7, 9]);
    let result = set.binary_search_by_key(&5, |&x| x);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct Set {
        elements: Vec<i32>,
    }

    impl Set {
        fn new(elements: Vec<i32>) -> Self {
            Set { elements }
        }

        fn as_slice(&self) -> &[i32] {
            &self.elements
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = Set::new(vec![1, 3, 5, 7, 9]);
    let result = set.binary_search_by_key(&6, |&x| x);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_key_empty() {
    struct Set {
        elements: Vec<i32>,
    }

    impl Set {
        fn new(elements: Vec<i32>) -> Self {
            Set { elements }
        }

        fn as_slice(&self) -> &[i32] {
            &self.elements
        }

        pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32) -> B,
            B: Ord,
        {
            self.as_slice().binary_search_by_key(b, f)
        }
    }

    let set = Set::new(vec![]);
    let result = set.binary_search_by_key(&1, |&x| x);
    assert_eq!(result, Err(0));
}

