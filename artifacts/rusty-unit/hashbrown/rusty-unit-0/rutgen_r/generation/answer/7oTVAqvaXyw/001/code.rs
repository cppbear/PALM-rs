// Answer 0

#[test]
fn test_fmt_empty() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, String>,
    }

    let test_map = TestStruct {
        inner: std::collections::HashMap::new(),
    };

    let mut output = String::new();
    write!(&mut output, "{:?}", test_map).unwrap();
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_entry() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, String>,
    }

    let mut inner_map = std::collections::HashMap::new();
    inner_map.insert(1, "value1".to_string());

    let test_map = TestStruct { inner: inner_map };

    let mut output = String::new();
    write!(&mut output, "{:?}", test_map).unwrap();
    assert_eq!(output, "[1]");
}

#[test]
fn test_fmt_multiple_entries() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, String>,
    }

    let mut inner_map = std::collections::HashMap::new();
    inner_map.insert(1, "value1".to_string());
    inner_map.insert(2, "value2".to_string());
    inner_map.insert(3, "value3".to_string());

    let test_map = TestStruct { inner: inner_map };

    let mut output = String::new();
    write!(&mut output, "{:?}", test_map).unwrap();
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_negative_keys() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, String>,
    }

    let mut inner_map = std::collections::HashMap::new();
    inner_map.insert(-1, "neg_value".to_string());
    inner_map.insert(-2, "neg_value2".to_string());

    let test_map = TestStruct { inner: inner_map };

    let mut output = String::new();
    write!(&mut output, "{:?}", test_map).unwrap();
    assert_eq!(output, "[-1, -2]");
}

#[test]
fn test_fmt_duplicate_keys() {
    struct TestStruct {
        inner: std::collections::HashMap<i32, String>,
    }

    let mut inner_map = std::collections::HashMap::new();
    inner_map.insert(1, "value1".to_string());
    inner_map.insert(1, "value2".to_string());

    let test_map = TestStruct { inner: inner_map };

    let mut output = String::new();
    write!(&mut output, "{:?}", test_map).unwrap();
    assert_eq!(output, "[1]");
}

