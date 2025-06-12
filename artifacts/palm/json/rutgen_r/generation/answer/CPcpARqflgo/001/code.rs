// Answer 0

#[derive(Debug)]
enum Type {
    Array,
    Other,
}

#[derive(Debug)]
enum Value {
    Array(Vec<i32>),
    Other,
}

impl Value {
    fn index_or_insert<'v>(&self, _: usize, v: &'v mut Value) -> &'v mut Value {
        match v {
            Value::Array(vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of JSON array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of JSON {}", self, Type::Other),
        }
    }
}

#[test]
#[should_panic(expected = "cannot access index")]
fn test_index_or_insert_with_non_array_value() {
    let index = 0;
    let mut value = Value::Other; // v matches Value::Array(vec) is false
    value.index_or_insert(index, &mut value);
}

#[test]
#[should_panic(expected = "cannot access index")]
fn test_index_or_insert_with_empty_array() {
    let index = 0;
    let mut value = Value::Array(Vec::new()); // An empty array scenario
    value.index_or_insert(index, &mut value);
}

