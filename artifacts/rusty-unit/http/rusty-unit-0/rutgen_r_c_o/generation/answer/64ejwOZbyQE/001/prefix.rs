// Answer 0

#[test]
fn test_extension_with_string() {
    let builder = Builder::new();
    let response_builder = builder.extension("Test Extension".to_string());
}

#[test]
fn test_extension_with_empty_string() {
    let builder = Builder::new();
    let response_builder = builder.extension("".to_string());
}

#[test]
fn test_extension_with_large_string() {
    let large_string = "x".repeat(1024);
    let builder = Builder::new();
    let response_builder = builder.extension(large_string);
}

#[test]
fn test_extension_with_integer() {
    let builder = Builder::new();
    let response_builder = builder.extension(42);
}

#[test]
fn test_extension_with_floating_point() {
    let builder = Builder::new();
    let response_builder = builder.extension(3.14);
}

#[test]
fn test_extension_with_struct() {
    #[derive(Clone, Debug)]
    struct MyStruct {
        value: i32,
    }
    
    let builder = Builder::new();
    let response_builder = builder.extension(MyStruct { value: 10 });
}

#[test]
fn test_extension_with_tuple() {
    let builder = Builder::new();
    let response_builder = builder.extension((1, "Tuple Extension"));
}

