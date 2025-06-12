// Answer 0

#[test]
fn test_get_index_valid_index_zero() {
    let slice = Box::new(Slice::new());
    let result = slice.get_index(0);
}

#[test]
fn test_get_index_valid_index_mid() {
    let elements = vec![
        Bucket { hash: HashValue::new(), key: 1, value: "a" },
        Bucket { hash: HashValue::new(), key: 2, value: "b" },
        Bucket { hash: HashValue::new(), key: 3, value: "c" },
    ];
    let slice = Box::new(Slice::from_slice(&elements));
    let result = slice.get_index(1);
}

#[test]
fn test_get_index_valid_index_last() {
    let elements = vec![
        Bucket { hash: HashValue::new(), key: 1, value: "a" },
        Bucket { hash: HashValue::new(), key: 2, value: "b" },
        Bucket { hash: HashValue::new(), key: 3, value: "c" },
    ];
    let slice = Box::new(Slice::from_slice(&elements));
    let result = slice.get_index(2);
}

#[test]
fn test_get_index_out_of_bounds() {
    let elements = vec![
        Bucket { hash: HashValue::new(), key: 1, value: "a" },
    ];
    let slice = Box::new(Slice::from_slice(&elements));
    let result = slice.get_index(1);
}

#[should_panic]
#[test]
fn test_get_index_negative_index() {
    let elements = vec![
        Bucket { hash: HashValue::new(), key: 1, value: "a" },
    ];
    let slice = Box::new(Slice::from_slice(&elements));
    let _result = slice.get_index(-1);
}

