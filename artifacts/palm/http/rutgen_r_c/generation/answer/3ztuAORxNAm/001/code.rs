// Answer 0

#[test]
fn test_extension_with_string() {
    let builder = Builder::new();
    let req = builder.extension("My Extension").body(()).unwrap();
    assert_eq!(req.extensions().get::<&'static str>(), Some(&"My Extension"));
}

#[test]
fn test_extension_with_integer() {
    let builder = Builder::new();
    let req = builder.extension(42).body(()).unwrap();
    assert_eq!(req.extensions().get::<i32>(), Some(&42));
}

#[test]
fn test_extension_with_struct() {
    #[derive(Clone)]
    struct MyStruct {
        value: i32,
    }

    let builder = Builder::new();
    let extension_value = MyStruct { value: 100 };
    let req = builder.extension(extension_value).body(()).unwrap();
    assert_eq!(req.extensions().get::<MyStruct>(), Some(&MyStruct { value: 100 }));
}

#[test]
fn test_extension_with_vec() {
    let builder = Builder::new();
    let extension_value = vec![1, 2, 3];
    let req = builder.extension(extension_value).body(()).unwrap();
    assert_eq!(req.extensions().get::<Vec<i32>>(), Some(&vec![1, 2, 3]));
}

#[should_panic]
fn test_extension_with_invalid_type() {
    let builder = Builder::new();
    // Attempting to retrieve an extension not set should panic
    let _ = builder.extension("My Extension").body(()).unwrap().extensions().get::<f64>();
}

