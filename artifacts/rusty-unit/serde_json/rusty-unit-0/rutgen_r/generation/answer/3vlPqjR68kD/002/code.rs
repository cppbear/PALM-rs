// Answer 0

#[derive(Default)]
struct TestIter {
    values: Vec<i32>,
}

impl Iterator for TestIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            None
        } else {
            Some(self.values.remove(0))
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.values.len();
        (len, Some(len))
    }
}

struct TestStruct {
    iter: TestIter,
}

impl Default for TestStruct {
    fn default() -> Self {
        Self { iter: TestIter::default() }
    }
}

#[test]
fn test_size_hint_equal_bounds() {
    let mut test_struct = TestStruct::default();
    test_struct.iter.values = vec![1, 2, 3];

    let result = test_struct.size_hint();
    assert_eq!(result, Some(3));
}

#[test]
fn test_size_hint_empty_iter() {
    let mut test_struct = TestStruct::default();
    test_struct.iter.values = Vec::new();

    let result = test_struct.size_hint();
    assert_eq!(result, Some(0));
}

#[test]
fn test_size_hint_single_element() {
    let mut test_struct = TestStruct::default();
    test_struct.iter.values = vec![42];

    let result = test_struct.size_hint();
    assert_eq!(result, Some(1));
}

#[test]
fn test_size_hint_non_matching_bounds() {
    let mut test_struct = TestStruct::default();
    test_struct.iter.values = vec![1, 2, 3, 4, 5];

    let result = test_struct.size_hint();
    assert_eq!(result, Some(5));
}

