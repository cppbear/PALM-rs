// Answer 0

#[derive(Clone)]
struct MyStruct {
    value: i32,
}

trait AnyClone: Clone {}

impl AnyClone for MyStruct {}

#[test]
fn test_clone_box() {
    let original = MyStruct { value: 42 };
    let cloned_box = original.clone_box();

    let cloned: &MyStruct = cloned_box.downcast_ref::<MyStruct>().unwrap();
    assert_eq!(cloned.value, 42);
}

#[test]
fn test_clone_box_empty() {
    let original = MyStruct { value: 0 };
    let cloned_box = original.clone_box();

    let cloned: &MyStruct = cloned_box.downcast_ref::<MyStruct>().unwrap();
    assert_eq!(cloned.value, 0);
}

