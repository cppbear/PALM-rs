// Answer 0

#[test]
fn test_iterator_len_hint_case_1() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.data.len() - self.index, Some(self.data.len() - self.index + 1))
        }
    }

    let iter = TestIterator { data: vec![1, 2, 3], index: 0 };
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_case_2() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.data.len() - self.index + 1, Some(self.data.len() - self.index + 2))
        }
    }

    let iter = TestIterator { data: vec![1, 2], index: 0 };
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_case_3() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(1))
        }
    }

    let iter = TestIterator { data: vec![], index: 0 };
    iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_case_4() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(2))
        }
    }

    let iter = TestIterator { data: vec![10], index: 0 };
    iterator_len_hint(&iter);
}

