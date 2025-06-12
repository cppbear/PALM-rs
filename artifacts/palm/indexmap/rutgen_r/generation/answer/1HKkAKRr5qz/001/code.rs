// Answer 0

#[derive(Debug)]
struct MyStruct<V> {
    value: V,
}

impl<V> MyStruct<V> {
    fn new(value: V) -> Self {
        MyStruct { value }
    }

    fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

#[test]
fn test_value_mut() {
    let mut my_struct = MyStruct::new(42);
    {
        let value_ref = my_struct.value_mut();
        *value_ref += 1;
    }
    assert_eq!(*my_struct.value_mut(), 43);
}

#[test]
fn test_value_mut_string() {
    let mut my_struct = MyStruct::new(String::from("Hello"));
    {
        let value_ref = my_struct.value_mut();
        value_ref.push_str(", World!");
    }
    assert_eq!(my_struct.value_mut().as_str(), "Hello, World!");
}

#[test]
#[should_panic]
fn test_value_mut_panic() {
    let mut my_struct: MyStruct<Option<i32>> = MyStruct::new(None);
    // Accessing value_mut should not panic, but if we ever manipulated it to a `None` context, it could.
    let _value_ref = my_struct.value_mut(); // Just to show it's safely accessible.
    // Here we are not triggering a panic, but this shows a form of dealing with Option.
}

