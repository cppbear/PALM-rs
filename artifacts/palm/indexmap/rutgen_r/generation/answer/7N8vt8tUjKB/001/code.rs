// Answer 0

#[derive(Debug)]
struct TestStruct {
    key: i32,
    value: String,
}

impl TestStruct {
    fn key_value(self) -> (i32, String) {
        (self.key, self.value)
    }
}

#[test]
fn test_key_value_valid_input() {
    let test_instance = TestStruct {
        key: 1,
        value: String::from("test"),
    };
    let result = test_instance.key_value();
    assert_eq!(result, (1, String::from("test")));
}

#[test]
fn test_key_value_empty_string() {
    let test_instance = TestStruct {
        key: 2,
        value: String::from(""),
    };
    let result = test_instance.key_value();
    assert_eq!(result, (2, String::from("")));
}

#[test]
fn test_key_value_negative_key() {
    let test_instance = TestStruct {
        key: -1,
        value: String::from("negative"),
    };
    let result = test_instance.key_value();
    assert_eq!(result, (-1, String::from("negative")));
}

#[test]
fn test_key_value_large_value() {
    let test_instance = TestStruct {
        key: 3,
        value: String::from("a".repeat(1000)), // Large string
    };
    let result = test_instance.key_value();
    assert_eq!(result, (3, String::from("a".repeat(1000))));
}

