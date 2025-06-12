// Answer 0

#[derive(Debug)]
struct TestIter {
    data: Vec<i32>,
    index: usize,
}

impl TestIter {
    fn new(data: Vec<i32>) -> Self {
        TestIter { data, index: 0 }
    }
}

impl Iterator for TestIter {
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
        let remaining = self.data.len() - self.index;
        (remaining, Some(remaining))
    }
}

struct TestStruct {
    iter: TestIter,
}

impl TestStruct {
    fn new(data: Vec<i32>) -> Self {
        TestStruct {
            iter: TestIter::new(data),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

#[test]
fn test_size_hint_exact_boundaries() {
    let test_struct = TestStruct::new(vec![1, 2, 3]);
    assert_eq!(test_struct.size_hint(), Some(3));
}

#[test]
fn test_size_hint_non_exact_boundaries() {
    let test_struct = TestStruct::new(vec![1, 2, 3, 4]);
    let _ = test_struct.iter.next(); // consuming one item
    assert_eq!(test_struct.size_hint(), None);
}

#[test]
fn test_size_hint_empty() {
    let test_struct = TestStruct::new(vec![]);
    assert_eq!(test_struct.size_hint(), Some(0));
}

