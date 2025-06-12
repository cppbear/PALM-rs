// Answer 0

#[test]
fn test_new_with_empty_vec() {
    let vec: Vec<Value> = vec![];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_single_null() {
    let vec = vec![Value::Null];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_single_bool() {
    let vec = vec![Value::Bool(true)];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_single_number() {
    let vec = vec![Value::Number(Number::from(42))];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_single_string() {
    let vec = vec![Value::String("test string".to_owned())];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_single_array() {
    let vec = vec![Value::Array(vec![Value::Null])];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_single_object() {
    let mut obj = Map::new();
    obj.insert("key".to_owned(), Value::Bool(false));
    let vec = vec![Value::Object(obj)];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_multiple_value_variants() {
    let vec = vec![
        Value::Null,
        Value::Bool(false),
        Value::Number(Number::from(3.14)),
        Value::String("example".to_owned()),
        Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]),
        {
            let mut obj = Map::new();
            obj.insert("a".to_owned(), Value::String("value".to_owned()));
            Value::Object(obj)
        }
    ];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_nested_values_depth_1() {
    let vec = vec![
        Value::Array(vec![Value::Number(Number::from(1))]),
        Value::Object({
            let mut obj = Map::new();
            obj.insert("inner".to_owned(), Value::String("nested".to_owned()));
            obj
        }),
    ];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_nested_values_depth_2() {
    let vec = vec![
        Value::Array(vec![
            Value::Array(vec![Value::Number(Number::from(1))]),
            Value::Object({
                let mut obj = Map::new();
                obj.insert("inner".to_owned(), Value::String("nested".to_owned()));
                obj
            }),
        ]),
    ];
    let deserializer = SeqDeserializer::new(vec);
}

#[test]
fn test_new_with_large_vector() {
    let mut vec = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec.push(Value::Number(Number::from(i)));
    }
    let deserializer = SeqDeserializer::new(vec);
} 

#[test]
fn test_new_with_varied_nested_structures() {
    let vec = vec![
        Value::Array(vec![Value::Array(vec![Value::Number(Number::from(1))])]),
        Value::Object({
            let mut obj = Map::new();
            obj.insert("outer".to_owned(), Value::Array(vec![Value::Number(Number::from(42))]));
            obj
        }),
    ];
    let deserializer = SeqDeserializer::new(vec);
}

