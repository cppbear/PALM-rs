// Answer 0

#[test]
fn test_iterator_len_hint_none_case() {
    struct TestIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len() - self.index, Some(self.items.len() + 1)) // lo != hi
        }
    }

    let iter = TestIterator { items: vec![1, 2, 3], index: 0 };
    assert_eq!(iterator_len_hint(&iter), None);
}

#[test]
fn test_iterator_len_hint_another_none_case() {
    struct AnotherTestIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for AnotherTestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.items.len(), Some(self.items.len() - 1)) // lo != hi
        }
    }

    let iter = AnotherTestIterator { items: vec![10, 20, 30, 40], index: 0 };
    assert_eq!(iterator_len_hint(&iter), None);
}

