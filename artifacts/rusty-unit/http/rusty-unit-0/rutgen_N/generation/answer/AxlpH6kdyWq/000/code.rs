// Answer 0

#[derive(Debug)]
struct MyStruct {
    value: String,
}

impl AsRef<str> for MyStruct {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[test]
fn test_display_impl() {
    let my_struct = MyStruct { value: String::from("Hello, World!") };
    let result = format!("{}", my_struct);
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_empty_string() {
    let my_struct = MyStruct { value: String::from("") };
    let result = format!("{}", my_struct);
    assert_eq!(result, "");
}

#[test]
fn test_special_characters() {
    let my_struct = MyStruct { value: String::from("!@#$%^&*()") };
    let result = format!("{}", my_struct);
    assert_eq!(result, "!@#$%^&*()");
}

