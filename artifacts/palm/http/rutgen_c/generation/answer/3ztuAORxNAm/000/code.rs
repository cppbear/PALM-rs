// Answer 0

#[test]
fn test_extension_inserts_string_extension() {
    use std::any::Any;
    use crate::{Builder, Extensions};

    let builder = Builder::new();
    let extension_value = "My Extension".to_string();

    let updated_builder = builder.extension(extension_value.clone());

    let extensions = updated_builder.extensions_ref().unwrap();
    assert_eq!(extensions.get::<String>(), Some(&extension_value));
}

#[test]
fn test_extension_inserts_integer_extension() {
    use std::any::Any;
    use crate::{Builder, Extensions};

    let builder = Builder::new();
    let extension_value = 42;

    let updated_builder = builder.extension(extension_value.clone());

    let extensions = updated_builder.extensions_ref().unwrap();
    assert_eq!(extensions.get::<i32>(), Some(&extension_value));
}

#[test]
fn test_extension_inserts_struct_extension() {
    use std::any::Any;
    use crate::{Builder, Extensions};

    #[derive(Clone)]
    struct MyStruct;

    let builder = Builder::new();
    let extension_value = MyStruct;

    let updated_builder = builder.extension(extension_value.clone());

    let extensions = updated_builder.extensions_ref().unwrap();
    assert_eq!(extensions.get::<MyStruct>(), Some(&extension_value));
}

#[test]
fn test_extension_handles_multiple_extensions() {
    use std::any::Any;
    use crate::{Builder, Extensions};

    let builder = Builder::new();
    let string_extension = "String Extension".to_string();
    let int_extension = 100;

    let builder_with_extensions = builder
        .extension(string_extension.clone())
        .extension(int_extension.clone());

    let extensions = builder_with_extensions.extensions_ref().unwrap();
    assert_eq!(extensions.get::<String>(), Some(&string_extension));
    assert_eq!(extensions.get::<i32>(), Some(&int_extension));
}

