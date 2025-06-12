// Answer 0

#[derive(Debug)]
struct MySet {
    elements: Vec<i32>,
}

impl MySet {
    fn as_slice(&self) -> &[i32] {
        &self.elements
    }

    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&i32) -> bool,
    {
        self.as_slice().partition_point(pred)
    }
}

#[test]
fn test_partition_point_empty() {
    let set = MySet { elements: Vec::new() };
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_all_less() {
    let set = MySet { elements: vec![1, 2, 3, 4] };
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 4);
}

#[test]
fn test_partition_point_all_greater() {
    let set = MySet { elements: vec![6, 7, 8, 9] };
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_mixed() {
    let set = MySet { elements: vec![1, 2, 3, 5, 6, 7] };
    let result = set.partition_point(|&x| x < 5);
    assert_eq!(result, 3);
}

#[test]
fn test_partition_point_boundary() {
    let set = MySet { elements: vec![1, 2, 2, 3, 4] };
    let result = set.partition_point(|&x| x <= 2);
    assert_eq!(result, 3);
}

