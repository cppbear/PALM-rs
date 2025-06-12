// Answer 0

#[test]
fn test_get_or_insert_default_with_integer_type() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_default::<i32>();
    assert_eq!(*value, 0); // Default value for i32 is 0
    *value += 5; // Mutate the value
    assert_eq!(*ext.get::<i32>().unwrap(), 5); // Check if the value was updated correctly
}

#[test]
fn test_get_or_insert_default_with_string_type() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_default::<String>();
    assert_eq!(*value, ""); // Default value for String is an empty string
    value.push_str("Hello");
    assert_eq!(*ext.get::<String>().unwrap(), "Hello"); // Check if the value was updated correctly
}

#[test]
fn test_get_or_insert_default_with_vec_type() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_default::<Vec<i32>>();
    assert_eq!(value.len(), 0); // Default value for Vec is empty
    value.push(1);
    assert_eq!(*ext.get::<Vec<i32>>().unwrap(), vec![1]); // Check if the value was updated correctly
}

#[test]
fn test_get_or_insert_default_with_struct() {
    #[derive(Default, Clone, Debug, PartialEq)]
    struct MyStruct {
        data: i32,
    }
    
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_default::<MyStruct>();
    assert_eq!(*value, MyStruct { data: 0 }); // Default value for MyStruct should be its default constructor
    value.data += 10;
    assert_eq!(*ext.get::<MyStruct>().unwrap(), MyStruct { data: 10 }); // Check if the value was updated correctly
}

