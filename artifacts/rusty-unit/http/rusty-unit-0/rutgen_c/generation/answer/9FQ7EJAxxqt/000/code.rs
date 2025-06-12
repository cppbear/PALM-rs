// Answer 0

#[test]
fn test_get_or_insert_default_with_default_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_default::<i32>() += 2;

    assert_eq!(*ext.get::<i32>().unwrap(), 2);
}

#[test]
fn test_get_or_insert_default_multiple_calls() {
    let mut ext = Extensions::new();
    let val1 = ext.get_or_insert_default::<String>();
    val1.push_str("Hello");

    let val2 = ext.get_or_insert_default::<String>();
    assert_eq!(val1, val2);
    assert_eq!(*val1, "Hello");
}

#[test]
fn test_get_or_insert_default_for_float() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_default::<f64>() += 3.5;

    assert_eq!(*ext.get::<f64>().unwrap(), 3.5);
}

#[test]
fn test_get_or_insert_default_for_struct() {
    #[derive(Default, Clone)]
    struct MyStruct {
        data: i32,
    }

    let mut ext = Extensions::new();
    ext.get_or_insert_default::<MyStruct>().data += 1;

    assert_eq!(ext.get::<MyStruct>().unwrap().data, 1);
}

#[test]
fn test_get_or_insert_default_when_empty() {
    let ext = Extensions::new();
    assert!(ext.get::<i32>().is_none());
}

#[test]
fn test_get_or_insert_default_with_multiple_types() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_default::<i32>() += 1;
    *ext.get_or_insert_default::<String>() += "Test";

    assert_eq!(*ext.get::<i32>().unwrap(), 1);
    assert_eq!(*ext.get::<String>().unwrap(), "Test");
}

