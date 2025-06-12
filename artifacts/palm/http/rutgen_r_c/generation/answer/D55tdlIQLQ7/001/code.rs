// Answer 0

#[test]
fn test_into_any_with_integer() {
    let value: Box<i32> = Box::new(42);
    let result: Box<dyn Any> = value.into_any();
    assert_eq!(*result.downcast::<i32>().unwrap(), 42);
}

#[test]
fn test_into_any_with_string() {
    let value: Box<String> = Box::new(String::from("Hello"));
    let result: Box<dyn Any> = value.into_any();
    assert_eq!(*result.downcast::<String>().unwrap(), String::from("Hello"));
}

#[test]
fn test_into_any_with_vector() {
    let value: Box<Vec<i32>> = Box::new(vec![1, 2, 3]);
    let result: Box<dyn Any> = value.into_any();
    assert_eq!(*result.downcast::<Vec<i32>>().unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_into_any_with_struct() {
    struct MyStruct {
        data: i32,
    }
    let value: Box<MyStruct> = Box::new(MyStruct { data: 10 });
    let result: Box<dyn Any> = value.into_any();
    let my_struct = result.downcast::<MyStruct>().unwrap();
    assert_eq!(my_struct.data, 10);
}

#[test]
fn test_into_any_with_empty_string() {
    let value: Box<String> = Box::new(String::new());
    let result: Box<dyn Any> = value.into_any();
    assert_eq!(*result.downcast::<String>().unwrap(), String::new());
}

