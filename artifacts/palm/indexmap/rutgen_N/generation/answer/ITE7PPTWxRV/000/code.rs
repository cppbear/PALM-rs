// Answer 0

#[derive(Debug)]
struct TestStruct<V> {
    value: V,
}

impl<V> TestStruct<V> {
    fn new(value: V) -> Self {
        TestStruct { value }
    }

    fn value(self) -> V {
        self.value
    }
}

#[test]
fn test_value_with_integer() {
    let test_struct = TestStruct::new(42);
    assert_eq!(test_struct.value(), 42);
}

#[test]
fn test_value_with_string() {
    let test_struct = TestStruct::new(String::from("Hello"));
    assert_eq!(test_struct.value(), "Hello");
}

#[test]
fn test_value_with_floating_point() {
    let test_struct = TestStruct::new(3.14);
    assert_eq!(test_struct.value(), 3.14);
}

