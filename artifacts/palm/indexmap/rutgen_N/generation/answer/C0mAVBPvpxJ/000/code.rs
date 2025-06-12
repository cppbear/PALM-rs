// Answer 0

#[derive(Debug)]
struct MyStruct<V> {
    value: V,
}

impl<V> MyStruct<V> {
    fn new(value: V) -> Self {
        MyStruct { value }
    }

    fn value_ref(&self) -> &V {
        &self.value
    }
}

#[test]
fn test_value_ref() {
    let my_struct = MyStruct::new(42);
    assert_eq!(*my_struct.value_ref(), 42);
}

#[test]
fn test_value_ref_with_string() {
    let my_struct = MyStruct::new(String::from("Hello"));
    assert_eq!(my_struct.value_ref(), &"Hello");
}

#[test]
fn test_value_ref_boundary_condition() {
    let my_struct = MyStruct::new(vec![1, 2, 3]);
    assert_eq!(my_struct.value_ref(), &vec![1, 2, 3]);
}

