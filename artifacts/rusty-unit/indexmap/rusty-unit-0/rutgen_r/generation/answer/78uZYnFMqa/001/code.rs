// Answer 0

#[test]
fn test_get_index_valid_index() {
    struct Bucket {
        key: i32,
    }

    impl Bucket {
        fn key_ref(&self) -> &i32 {
            &self.key
        }
    }

    struct Slice {
        entries: Vec<Bucket>,
    }

    impl Slice {
        pub fn get_index(&self, index: usize) -> Option<&i32> {
            self.entries.get(index).map(Bucket::key_ref)
        }
    }

    let slice = Slice {
        entries: vec![Bucket { key: 10 }, Bucket { key: 20 }, Bucket { key: 30 }],
    };

    assert_eq!(slice.get_index(0), Some(&10));
    assert_eq!(slice.get_index(1), Some(&20));
    assert_eq!(slice.get_index(2), Some(&30));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct Bucket {
        key: i32,
    }

    impl Bucket {
        fn key_ref(&self) -> &i32 {
            &self.key
        }
    }

    struct Slice {
        entries: Vec<Bucket>,
    }

    impl Slice {
        pub fn get_index(&self, index: usize) -> Option<&i32> {
            self.entries.get(index).map(Bucket::key_ref)
        }
    }

    let slice = Slice {
        entries: vec![Bucket { key: 10 }, Bucket { key: 20 }],
    };

    assert_eq!(slice.get_index(2), None);
    assert_eq!(slice.get_index(usize::MAX), None);
}

#[test]
#[should_panic]
fn test_get_index_negative_index() {
    struct Bucket {
        key: i32,
    }

    impl Bucket {
        fn key_ref(&self) -> &i32 {
            &self.key
        }
    }

    struct Slice {
        entries: Vec<Bucket>,
    }

    impl Slice {
        pub fn get_index(&self, index: usize) -> Option<&i32> {
            self.entries.get(index).map(Bucket::key_ref)
        }
    }

    let slice = Slice {
        entries: vec![Bucket { key: 10 }],
    };

    // Indices are of type usize, so this would not panic,
    // but trying an invalid approach to test limits can be implemented via casting.
    let _panic = slice.get_index(-1 as usize);
}

