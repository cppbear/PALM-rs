// Answer 0

#[test]
fn test_iter_empty_set() {
    struct MockSet<T> {
        data: Vec<T>,
    }

    impl<T> MockSet<T> {
        fn new() -> Self {
            MockSet { data: Vec::new() }
        }

        fn as_entries(&self) -> &[T] {
            &self.data
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.as_entries().iter()
        }
    }

    let set: MockSet<i32> = MockSet::new();
    let mut iter = set.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_element_set() {
    struct MockSet<T> {
        data: Vec<T>,
    }

    impl<T> MockSet<T> {
        fn new() -> Self {
            MockSet { data: Vec::new() }
        }

        fn push(&mut self, value: T) {
            self.data.push(value);
        }

        fn as_entries(&self) -> &[T] {
            &self.data
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.as_entries().iter()
        }
    }

    let mut set: MockSet<i32> = MockSet::new();
    set.push(42);
    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(&42));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_elements_set() {
    struct MockSet<T> {
        data: Vec<T>,
    }

    impl<T> MockSet<T> {
        fn new() -> Self {
            MockSet { data: Vec::new() }
        }

        fn push(&mut self, value: T) {
            self.data.push(value);
        }

        fn as_entries(&self) -> &[T] {
            &self.data
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.as_entries().iter()
        }
    }

    let mut set: MockSet<i32> = MockSet::new();
    set.push(10);
    set.push(20);
    set.push(30);
    let mut iter = set.iter();
    
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert!(iter.next().is_none());
}

