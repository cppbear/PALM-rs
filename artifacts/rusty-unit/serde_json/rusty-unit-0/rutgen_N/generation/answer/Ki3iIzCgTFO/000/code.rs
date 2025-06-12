// Answer 0

#[derive(Debug)]
struct Value {
    vec: Vec<i32>,
}

impl Value {
    fn Array(vec: Vec<i32>) -> Self {
        Value { vec }
    }
}

#[derive(Debug)]
struct Serializer {
    vec: Vec<i32>,
}

impl Serializer {
    fn new() -> Self {
        Serializer { vec: vec![] }
    }

    fn end(self) -> Result<Value, &'static str> {
        Ok(Value::Array(self.vec))
    }
}

#[test]
fn test_end_with_empty_vector() {
    let serializer = Serializer::new();
    let result = serializer.end().unwrap();
    assert_eq!(result.vec, vec![]);
}

#[test]
fn test_end_with_non_empty_vector() {
    let mut serializer = Serializer::new();
    serializer.vec.push(1);
    serializer.vec.push(2);
    serializer.vec.push(3);
    
    let result = serializer.end().unwrap();
    assert_eq!(result.vec, vec![1, 2, 3]);
}

