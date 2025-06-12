// Answer 0

#[test]
fn test_deref_borrowed() {
    struct TestStruct {
        value: i32,
    }

    let borrowed_value = TestStruct { value: 42 };
    let reference = Reference::Borrowed(&borrowed_value);

    assert_eq!(reference.deref().value, 42);
}

#[test]
fn test_deref_copied() {
    struct TestStruct {
        value: i32,
    }

    let copied_value = TestStruct { value: 99 };
    let reference = Reference::Copied(&copied_value);

    assert_eq!(reference.deref().value, 99);
}

#[test]
fn test_deref_multiple_references() {
    struct TestStruct {
        value: i32,
    }

    let borrowed_value = TestStruct { value: 1 };
    let copied_value = TestStruct { value: 2 };

    let borrowed_reference = Reference::Borrowed(&borrowed_value);
    let copied_reference = Reference::Copied(&copied_value);

    assert_eq!(borrowed_reference.deref().value, 1);
    assert_eq!(copied_reference.deref().value, 2);
}

