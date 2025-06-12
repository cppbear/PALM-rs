// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[test]
fn test_as_any_mut() {
    let mut my_struct = MyStruct;
    let any_ref = my_struct.as_any_mut();
    assert!(any_ref.is::<MyStruct>());
}

#[test]
fn test_as_any_mut_multiple_calls() {
    let mut my_struct = MyStruct;
    let any_ref1 = my_struct.as_any_mut();
    let any_ref2 = my_struct.as_any_mut();
    assert!(any_ref1.is::<MyStruct>());
    assert!(any_ref2.is::<MyStruct>());
    assert_eq!(any_ref1 as *mut _ as usize, any_ref2 as *mut _ as usize);
}

