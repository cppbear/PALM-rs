// Answer 0

#[test]
fn test_header_with_valid_name_and_value() {
    let builder = Builder::new();
    let req_builder = builder.header("Accept", "text/html");
}

#[test]
fn test_header_with_custom_name_and_value() {
    let builder = Builder::new();
    let req_builder = builder.header("X-Custom-Foo", "bar");
}

#[test]
fn test_header_with_long_name() {
    let long_name = "A".repeat(256);
    let builder = Builder::new();
    let req_builder = builder.header(long_name, "valid_value");
}

#[test]
fn test_header_with_long_value() {
    let builder = Builder::new();
    let long_value = "B".repeat(4096);
    let req_builder = builder.header("Valid-Name", long_value);
}

#[test]
fn test_header_with_empty_name() {
    let builder = Builder::new();
    let req_builder = builder.header("", "value");
}

#[test]
#[should_panic]
fn test_header_with_control_character_in_name() {
    let builder = Builder::new();
    let req_builder = builder.header("Invalid\x00Name", "value");
}

#[test]
#[should_panic]
fn test_header_with_control_character_in_value() {
    let builder = Builder::new();
    let req_builder = builder.header("Valid-Name", "Invalid\x00Value");
}

