// Answer 0

#[test]
fn test_as_slice_with_non_empty_set() {
    struct MySet<T> {
        entries: Vec<T>,
    }

    impl<T> MySet<T> {
        fn new() -> Self {
            MySet { entries: Vec::new() }
        }

        fn as_entries(&self) -> &[T] {
            &self.entries
        }

        fn as_slice(&self) -> &[T] {
            &self.as_entries()
        }

        fn insert(&mut self, value: T) {
            self.entries.push(value);
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let slice = set.as_slice();
    assert_eq!(slice, &[1, 2, 3]);
}

#[test]
fn test_as_slice_with_empty_set() {
    struct MySet<T> {
        entries: Vec<T>,
    }

    impl<T> MySet<T> {
        fn new() -> Self {
            MySet { entries: Vec::new() }
        }

        fn as_entries(&self) -> &[T] {
            &self.entries
        }

        fn as_slice(&self) -> &[T] {
            &self.as_entries()
        }
    }

    let set: MySet<i32> = MySet::new();

    let slice = set.as_slice();
    assert!(slice.is_empty());
}

