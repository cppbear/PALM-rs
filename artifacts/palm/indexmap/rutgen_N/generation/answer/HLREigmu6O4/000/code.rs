// Answer 0

#[derive(Debug, PartialEq)]
struct TestValue {
    value: i32,
}

struct TestSet {
    map: std::collections::HashMap<i32, TestValue>,
}

impl TestSet {
    fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn replace_full(&mut self, value: TestValue) -> (usize, Option<TestValue>) {
        let hash = value.value;
        match self.map.insert(hash, value) {
            Some(replaced) => (hash as usize, Some(replaced)),
            None => (hash as usize, None),
        }
    }
}

#[test]
fn test_replace_full_with_new_value() {
    let mut set = TestSet::new();
    let value = TestValue { value: 1 };
    let (index, replaced) = set.replace_full(value);
    assert_eq!(index, 1);
    assert_eq!(replaced, None);
}

#[test]
fn test_replace_full_with_existing_value() {
    let mut set = TestSet::new();
    let value1 = TestValue { value: 1 };
    let value2 = TestValue { value: 1 };
    let (index1, replaced1) = set.replace_full(value1);
    let (index2, replaced2) = set.replace_full(value2);
    assert_eq!(index1, 1);
    assert_eq!(index2, 1);
    assert_eq!(replaced1, None);
    assert_eq!(replaced2, Some(TestValue { value: 1 }));
}

#[test]
fn test_replace_full_with_different_value() {
    let mut set = TestSet::new();
    let value1 = TestValue { value: 1 };
    let value2 = TestValue { value: 2 };
    let (index1, replaced1) = set.replace_full(value1);
    let (index2, replaced2) = set.replace_full(value2);
    assert_eq!(index1, 1);
    assert_eq!(index2, 2);
    assert_eq!(replaced1, None);
    assert_eq!(replaced2, None);
}

