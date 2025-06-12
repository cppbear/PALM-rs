// Answer 0

#[derive(Debug)]
struct MyStruct {
    map: std::collections::HashMap<String, usize>,
}

impl MyStruct {
    fn new() -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert("apple".to_string(), 1);
        map.insert("banana".to_string(), 2);
        MyStruct { map }
    }

    pub fn get_full<Q>(&self, value: &Q) -> Option<(usize, &usize)>
    where
        Q: ?Sized + Hash + std::cmp::PartialEq + std::hash::Hash,
    {
        self.map.get_full(value).map(|(i, x, &())| (i, x))
    }
}

impl std::hash::Hash for MyStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (key, value) in &self.map {
            key.hash(state);
            value.hash(state);
        }
    }
}

#[test]
fn test_get_full_existing_key() {
    let my_struct = MyStruct::new();
    let result = my_struct.get_full(&"apple");
    assert_eq!(result, Some((0, &1)));
}

#[test]
fn test_get_full_non_existing_key() {
    let my_struct = MyStruct::new();
    let result = my_struct.get_full(&"orange");
    assert_eq!(result, None);
}

#[test]
fn test_get_full_boundary_conditions() {
    let my_struct = MyStruct::new();
    let result_empty = my_struct.get_full(&"");
    assert_eq!(result_empty, None);
}

