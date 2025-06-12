// Answer 0

#[test]
fn test_extension_with_valid_string() {
    let builder = Builder::new();
    builder.extension("Valid String");
}

#[test]
fn test_extension_with_integer() {
    let builder = Builder::new();
    builder.extension(123);
}

#[test]
fn test_extension_with_boolean() {
    let builder = Builder::new();
    builder.extension(true);
}

#[test]
fn test_extension_with_struct() {
    #[derive(Clone)]
    struct MyStruct;
    let builder = Builder::new();
    builder.extension(MyStruct);
}

#[test]
fn test_extension_with_long_string() {
    let long_string = "A".repeat(1024);
    let builder = Builder::new();
    builder.extension(long_string);
}

#[test]
#[should_panic]
fn test_extension_with_none() {
    let builder = Builder::new();
    builder.extension(None::<&str>);
}

