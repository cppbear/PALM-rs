// Answer 0

#[test]
fn test_first_with_non_empty_entries() {
    struct Bucket<T> {
        value: T,
    }

    struct Slice<T> {
        entries: Vec<Bucket<T>>,
    }

    impl<T> Slice<T> {
        pub fn first(&self) -> Option<&T> {
            self.entries.first().map(|bucket| &bucket.value)
        }
    }

    let slice = Slice {
        entries: vec![
            Bucket { value: 1 },
            Bucket { value: 2 },
        ],
    };

    assert_eq!(slice.first(), Some(&1));
}

#[test]
fn test_first_with_empty_entries() {
    struct Bucket<T> {
        value: T,
    }

    struct Slice<T> {
        entries: Vec<Bucket<T>>,
    }

    impl<T> Slice<T> {
        pub fn first(&self) -> Option<&T> {
            self.entries.first().map(|bucket| &bucket.value)
        }
    }

    let slice: Slice<i32> = Slice { entries: Vec::new() };

    assert_eq!(slice.first(), None);
}

