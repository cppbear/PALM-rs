// Answer 0

#[derive(Debug)]
struct TestStruct {
    value: u64,
}

struct TestTable {
    data: Vec<TestStruct>,
}

impl TestTable {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn insert(&mut self, value: u64) {
        self.data.push(TestStruct { value });
    }

    fn find(&self, hash: u64, mut eq: impl FnMut(&TestStruct) -> bool) -> Option<&TestStruct> {
        self.data.iter().find(|&item| hash == item.value && eq(item))
    }

    pub fn get(&self, hash: u64, eq: impl FnMut(&TestStruct) -> bool) -> Option<&TestStruct> {
        match self.find(hash, eq) {
            Some(bucket) => Some(unsafe { bucket }),
            None => None,
        }
    }
}

#[test]
fn test_get_existing_element() {
    let mut table = TestTable::new();
    table.insert(1);
    table.insert(2);
    table.insert(3);

    let result = table.get(2, |item| item.value == 2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 2);
}

#[test]
fn test_get_non_existing_element() {
    let mut table = TestTable::new();
    table.insert(1);
    table.insert(2);
    table.insert(3);

    let result = table.get(4, |item| item.value == 4);
    assert!(result.is_none());
}

#[test]
fn test_get_with_different_equality_function() {
    let mut table = TestTable::new();
    table.insert(10);
    table.insert(20);
    table.insert(30);

    let result = table.get(20, |item| item.value > 10);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 20);
}

#[test]
fn test_get_with_empty_table() {
    let table = TestTable::new();
    let result = table.get(5, |item| item.value == 5);
    assert!(result.is_none());
}

