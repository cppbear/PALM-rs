// Answer 0

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
}

impl Value {
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
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
            _ => panic!("cannot access index {} of JSON {}", self, Type(v)),
        }
    }
}

#[derive(Debug)]
struct Type<'a>(&'a Value);

#[test]
fn test_index_or_insert_valid_index() {
    let mut value = Value::Array(vec![Value::Array(vec![]), Value::Array(vec![Value::Array(vec![])])]);
    let index = 1;
    let result = index.index_or_insert(&mut value);
    assert_eq!(result, &mut value);
}

#[test]
#[should_panic(expected = "cannot access index 2 of JSON array of length 2")]
fn test_index_or_insert_out_of_bounds() {
    let mut value = Value::Array(vec![Value::Array(vec![]), Value::Array(vec![])]);
    let index = 2;
    index.index_or_insert(&mut value);
}

#[test]
#[should_panic(expected = "cannot access index 0 of JSON")]
fn test_index_or_insert_non_array() {
    let value = Value::Array(vec![]);
    let index = 0;
    index.index_or_insert(&mut value);
}

