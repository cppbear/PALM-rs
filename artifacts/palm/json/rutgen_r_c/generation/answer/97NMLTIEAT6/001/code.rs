// Answer 0

#[test]
fn test_deref_copied_variant() {
    struct ExampleStruct {
        value: i32,
    }

    // Initialize ExampleStruct using constructors
    let data = ExampleStruct { value: 42 };
    
    // Create a Reference::Copied instance
    let reference = Reference::Copied(&data);

    // Deref to obtain the inner value
    let result = reference.deref();

    // Verify that the result matches the expected output
    assert_eq!(result.value, 42);
}

#[test]
fn test_deref_borrowed_variant() {
    struct ExampleStruct {
        value: i32,
    }

    // Initialize ExampleStruct using constructors
    let data = ExampleStruct { value: 100 };
    
    // Create a Reference::Borrowed instance
    let reference = Reference::Borrowed(&data);

    // Deref to obtain the inner value
    let result = reference.deref();

    // Verify that the result matches the expected output
    assert_eq!(result.value, 100);
}

