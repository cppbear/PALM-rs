// Answer 0

#[derive(Debug)]
struct SimpleHashBuilder;

impl BuildHasher for SimpleHashBuilder {
    type Hasher = std::collections::hash_map::DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        std::collections::hash_map::DefaultHasher::new()
    }
}

#[derive(Debug)]
struct TestMutableKeys {
    map: IndexMap<i32, String, SimpleHashBuilder>,
}

impl TestMutableKeys {
    fn new() -> Self {
        let map = IndexMap::new();
        Self { map }
    }

    fn populate(&mut self) {
        self.map.insert(1, String::from("one"));
        self.map.insert(2, String::from("two"));
        self.map.insert(3, String::from("three"));
    }
}

#[test]
fn test_retain2_all_elements_kept() {
    let mut test_struct = TestMutableKeys::new();
    test_struct.populate();
    test_struct.map.retain2(|_key, _value| true);
}

#[test]
fn test_retain2_no_elements_kept() {
    let mut test_struct = TestMutableKeys::new();
    test_struct.populate();
    test_struct.map.retain2(|_key, _value| false);
}

#[test]
fn test_retain2_some_elements_kept() {
    let mut test_struct = TestMutableKeys::new();
    test_struct.populate();
    test_struct.map.retain2(|key, _value| *key % 2 == 0);
}

#[test]
fn test_retain2_empty_map() {
    let mut test_struct = TestMutableKeys::new();
    test_struct.map.retain2(|_key, _value| true);
}

#[test]
fn test_retain2_single_element_keep() {
    let mut test_struct = TestMutableKeys::new();
    test_struct.map.insert(1, String::from("one"));
    test_struct.map.retain2(|key, _value| *key == 1);
}

#[test]
fn test_retain2_multiple_calls() {
    let mut test_struct = TestMutableKeys::new();
    test_struct.populate();
    test_struct.map.retain2(|key, _value| *key > 1);
    test_struct.map.retain2(|key, _value| *key == 2);
}

