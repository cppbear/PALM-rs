// Answer 0

#[derive(Debug, Clone)]
struct Value {
    data: Vec<i32>,
}

impl Value {
    fn new() -> Self {
        Value { data: Vec::new() }
    }

    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        // Just for testing, mimic an index_or_insert logic
        if self.data.is_empty() {
            // Assuming we want to insert into the first index when empty
            return v; // Will return the reference to the value passed
        }
        // In a real scenario, you'd have proper indexing logic
        v
    }
}

#[test]
fn test_index_or_insert_empty() {
    let value = Value::new();
    let mut new_value = Value::new();
    let result = value.index_or_insert(&mut new_value);
    assert_eq!(result, &mut new_value);
}

#[test]
fn test_index_or_insert_non_empty() {
    let value = Value { data: vec![1, 2, 3] };
    let mut new_value = Value { data: vec![4] };
    let result = value.index_or_insert(&mut new_value);
    assert_eq!(result, &mut new_value);
}

