// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    key: T,
}

#[derive(Debug)]
struct Slice<T> {
    entries: Vec<Bucket<T>>,
}

impl<T> Slice<T> {
    pub fn first(&self) -> Option<&T> {
        self.entries.first().map(|bucket| &bucket.key)
    }
}

#[test]
fn test_first_with_non_empty_slice() {
    let slice = Slice {
        entries: vec![
            Bucket { key: 1 },
            Bucket { key: 2 },
            Bucket { key: 3 },
        ],
    };
    assert_eq!(slice.first(), Some(&1));
}

#[test]
fn test_first_with_empty_slice() {
    let slice: Slice<i32> = Slice { entries: vec![] };
    assert_eq!(slice.first(), None);
}

#[test]
fn test_first_with_single_element_slice() {
    let slice = Slice {
        entries: vec![Bucket { key: 42 }],
    };
    assert_eq!(slice.first(), Some(&42));
}

#[test]
fn test_first_with_string_keys() {
    let slice = Slice {
        entries: vec![
            Bucket { key: "first".to_string() },
            Bucket { key: "second".to_string() },
        ],
    };
    assert_eq!(slice.first(), Some(&"first".to_string()));
}

