// Answer 0

#[test]
fn test_value_mut_with_integer_value() {
    let mut bucket = Bucket {
        hash: HashValue(1),
        key: 42,
        value: 100,
    };
    let value_mut = bucket.value_mut();
    *value_mut += 1;
}

#[test]
fn test_value_mut_with_string_value() {
    let mut bucket = Bucket {
        hash: HashValue(2),
        key: String::from("key"),
        value: String::from("value"),
    };
    let value_mut = bucket.value_mut();
    value_mut.push_str(" updated");
}

#[test]
fn test_value_mut_with_float_value() {
    let mut bucket = Bucket {
        hash: HashValue(3),
        key: 3.14,
        value: 1.0,
    };
    let value_mut = bucket.value_mut();
    *value_mut *= 2.0;
}

#[test]
fn test_value_mut_with_vector_value() {
    let mut bucket = Bucket {
        hash: HashValue(4),
        key: 'a',
        value: vec![1, 2, 3],
    };
    let value_mut = bucket.value_mut();
    value_mut.push(4);
}

#[test]
fn test_value_mut_with_empty_string_value() {
    let mut bucket = Bucket {
        hash: HashValue(5),
        key: String::from("empty"),
        value: String::new(),
    };
    let value_mut = bucket.value_mut();
    value_mut.push('c');
}

