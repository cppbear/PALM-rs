// Answer 0

#[test]
fn test_get_index_valid() {
    struct Entry {
        key: i32,
        value: i32,
    }
    
    struct Bucket {
        entries: Vec<Entry>,
    }

    impl Bucket {
        fn key_ref(&self, index: usize) -> &i32 {
            &self.entries[index].key
        }
    }

    let bucket = Bucket {
        entries: vec![Entry { key: 1, value: 10 }, Entry { key: 2, value: 20 }],
    };

    assert_eq!(bucket.get_index(0), Some(&1));
    assert_eq!(bucket.get_index(1), Some(&2));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct Entry {
        key: i32,
        value: i32,
    }
    
    struct Bucket {
        entries: Vec<Entry>,
    }

    impl Bucket {
        fn key_ref(&self, index: usize) -> &i32 {
            &self.entries[index].key
        }
    }

    let bucket = Bucket {
        entries: vec![Entry { key: 1, value: 10 }],
    };

    assert_eq!(bucket.get_index(1), None);
} 

#[test]
fn test_get_index_empty() {
    struct Entry {
        key: i32,
        value: i32,
    }

    struct Bucket {
        entries: Vec<Entry>,
    }

    impl Bucket {
        fn key_ref(&self, index: usize) -> &i32 {
            &self.entries[index].key
        }
    }

    let bucket = Bucket { entries: vec![] };

    assert_eq!(bucket.get_index(0), None);
}

