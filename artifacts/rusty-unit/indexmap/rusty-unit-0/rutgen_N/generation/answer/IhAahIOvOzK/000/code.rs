// Answer 0

#[test]
fn test_last_with_non_empty() {
    struct TestMap {
        data: Vec<Bucket<i32>>,
    }

    struct Bucket<T> {
        key: T,
    }

    impl<T> TestMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn push(&mut self, key: T) {
            self.data.push(Bucket { key });
        }

        fn as_entries(&self) -> &Vec<Bucket<T>> {
            &self.data
        }

        pub fn last(&self) -> Option<&T> {
            self.as_entries().last().map(|bucket| &bucket.key)
        }
    }

    let mut map = TestMap::new();
    map.push(1);
    map.push(2);
    map.push(3);

    assert_eq!(map.last(), Some(&3));
}

#[test]
fn test_last_with_empty() {
    struct TestMap {
        data: Vec<Bucket<i32>>,
    }

    struct Bucket<T> {
        key: T,
    }

    impl<T> TestMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn as_entries(&self) -> &Vec<Bucket<T>> {
            &self.data
        }

        pub fn last(&self) -> Option<&T> {
            self.as_entries().last().map(|bucket| &bucket.key)
        }
    }

    let map = TestMap::new();

    assert_eq!(map.last(), None);
}

