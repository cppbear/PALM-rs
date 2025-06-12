// Answer 0

#[test]
fn test_unwrap_class_bytes_panic_on_non_classbytes() {
    // Create a struct to represent HirFrame with a non-ClassBytes variant
    struct NonClassBytes;

    // Create a variant of HirFrame that does not match ClassBytes
    enum HirFrame {
        NonClassBytesVariant(NonClassBytes),
        // Assume other variants exist
    }

    // Initialize a value of HirFrame that is not ClassBytes
    let frame = HirFrame::NonClassBytesVariant(NonClassBytes);

    // Check that calling unwrap_class_bytes panics
    let result = std::panic::catch_unwind(|| {
        match frame {
            HirFrame::NonClassBytesVariant(_) => {
                // This should panic
                unwrap_class_bytes(frame);
            },
            _ => unreachable!(),
        }
    });

    assert!(result.is_err());
}

#[test]
fn test_unwrap_class_bytes_panic_on_unmatching() {
    // Create a struct to represent HirFrame with a different non-ClassBytes variant
    struct AnotherNonClassBytes;

    // Create a variant of HirFrame that does not match ClassBytes
    enum HirFrame {
        AnotherVariant(AnotherNonClassBytes),
        // Assume other variants exist
    }

    // Initialize a value of HirFrame that is not ClassBytes
    let frame = HirFrame::AnotherVariant(AnotherNonClassBytes);

    // Check that calling unwrap_class_bytes panics
    let result = std::panic::catch_unwind(|| {
        match frame {
            HirFrame::AnotherVariant(_) => {
                // This should panic
                unwrap_class_bytes(frame);
            },
            _ => unreachable!(),
        }
    });

    assert!(result.is_err());
}

