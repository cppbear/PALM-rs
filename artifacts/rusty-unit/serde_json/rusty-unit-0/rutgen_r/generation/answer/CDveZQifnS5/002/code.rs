// Answer 0

#[derive(Debug)]
struct TestIter {
    data: Vec<i32>,
}

impl TestIter {
    fn new(data: Vec<i32>) -> Self {
        TestIter { data }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lower = self.data.len();
        let upper = Some(lower);
        (lower, upper)
    }

    fn iter(&self) -> std::slice::Iter<i32> {
        self.data.iter()
    }
}

#[test]
fn test_size_hint_equal_lower_upper() {
    let data = vec![1, 2, 3, 4, 5]; // Lower and upper are equal
    let test_iter = TestIter::new(data);
    let result = test_iter.size_hint();
    assert_eq!(result, (5, Some(5))); // Ensuring size_hint returns (5, Some(5))
    
    let upper_value = result.1.unwrap(); // upper should be 5
    let size_hint_result = test_iter.size_hint();
    assert_eq!(size_hint_result, Some(upper_value)); // Expected Some(5)
}

