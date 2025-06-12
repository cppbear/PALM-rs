// Answer 0

#[test]
fn test_swap_greed_disable() {
    // Define a minimal structure for TranslatorBuilder
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        swap_greed: Option<bool>,
    }

    // Initialize an instance of TranslatorBuilder
    let mut builder = TranslatorBuilder {
        flags: Flags { swap_greed: None },
    };

    // Call the method with `yes` set to false
    let result = builder.swap_greed(false);

    // Assert that the result is the same instance of TranslatorBuilder
    assert_eq!(result as *const _, &mut builder as *mut _);

    // Assert that swap_greed flag is set to None
    assert_eq!(builder.flags.swap_greed, None);
}

