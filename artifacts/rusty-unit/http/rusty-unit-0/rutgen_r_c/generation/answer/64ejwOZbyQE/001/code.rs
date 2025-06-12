// Answer 0

#[test]
fn test_extension_with_string() {
    let builder = Builder::new();
    let result = builder.extension("My Extension");
    let response_result = result.body(()); // Assuming body method does not panic
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert_eq!(response.extensions().get::<&'static str>(), Some(&"My Extension"));
}

#[test]
fn test_extension_with_integer() {
    let builder = Builder::new();
    let result = builder.extension(42);
    let response_result = result.body(()); // Assuming body method does not panic
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert_eq!(response.extensions().get::<i32>(), Some(&42));
}

#[test]
fn test_extension_with_struct() {
    #[derive(Clone)]
    struct MyStruct {
        value: i32,
    }

    let builder = Builder::new();
    let my_struct = MyStruct { value: 10 };
    let result = builder.extension(my_struct);
    let response_result = result.body(()); // Assuming body method does not panic
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert_eq!(response.extensions().get::<MyStruct>(), Some(&my_struct));
}

#[test]
#[should_panic]
fn test_extension_should_panic_due_to_send() {
    // A non-Send type will cause a panic when used with Send bound
    struct NonSend;

    let builder = Builder::new();
    let result = builder.extension(NonSend);
    let _ = result.body(()); // Assuming body method does not panic
}

#[test]
fn test_multiple_extensions() {
    let builder = Builder::new();
    let result = builder
        .extension("First Extension")
        .extension(100);
    let response_result = result.body(()); // Assuming body method does not panic
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    
    assert_eq!(response.extensions().get::<&'static str>(), Some(&"First Extension"));
    assert_eq!(response.extensions().get::<i32>(), Some(&100));
}

#[test]
fn test_no_extension() {
    let builder = Builder::new();
    let response_result = builder.body(()); // Assuming body method does not panic
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    
    assert!(response.extensions().is_empty());
}

