// Answer 0

#[test]
fn test_hash_non_empty_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl<'a> IntoIterator for &'a TestMap {
        type Item = (&'a i32, &'a String);
        type IntoIter = std::iter::Enumerate<std::slice::Iter<'a, (i32, String)>>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter().enumerate().map(|(i, (k, v))| (&k, &v))
        }
    }

    let mut hasher = DefaultHasher::new();
    let map = TestMap {
        data: vec![(1, String::from("one")), (2, String::from("two"))],
    };
    map.hash(&mut hasher);
    let result = hasher.finish();
    assert!(result > 0);
}

#[test]
fn test_hash_empty_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct EmptyMap {}

    impl EmptyMap {
        fn len(&self) -> usize {
            0
        }
    }

    impl<'a> IntoIterator for &'a EmptyMap {
        type Item = ();
        type IntoIter = std::iter::Empty<()>;

        fn into_iter(self) -> Self::IntoIter {
            std::iter::empty()
        }
    }

    let mut hasher = DefaultHasher::new();
    let map = EmptyMap {};
    map.hash(&mut hasher);
    let result = hasher.finish();
    assert_eq!(result, 0);
}

#[should_panic]
#[test]
fn test_hash_uninitialized_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct UninitializedMap;

    impl UninitializedMap {
        fn len(&self) -> usize {
            0 // This is deliberately misleading to trigger panic.
        }
    }

    impl<'a> IntoIterator for &'a UninitializedMap {
        type Item = ();
        type IntoIter = std::iter::Empty<()>;

        fn into_iter(self) -> Self::IntoIter {
            panic!("Attempted to iterate over uninitialized map");
        }
    }

    let mut hasher = DefaultHasher::new();
    let map = UninitializedMap {};
    map.hash(&mut hasher);
}

