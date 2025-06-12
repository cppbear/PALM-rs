// Answer 0

#[test]
fn test_append_string_zero_length() {
    use rand::rngs::OsRng; // A random number generator.

    let alphanumeric = Alphanumeric;
    let mut rng = OsRng;
    let mut output = String::new();

    // Test with length 0, should not modify the output.
    alphanumeric.append_string(&mut rng, &mut output, 0);
    assert_eq!(output, "");
}

#[test]
fn test_append_string_small_length() {
    use rand::rngs::OsRng;

    let alphanumeric = Alphanumeric;
    let mut rng = OsRng;
    let mut output = String::new();

    // Test with a small length.
    alphanumeric.append_string(&mut rng, &mut output, 5);
    assert_eq!(output.len(), 5);
    assert!(output.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_append_string_large_length() {
    use rand::rngs::OsRng;

    let alphanumeric = Alphanumeric;
    let mut rng = OsRng;
    let mut output = String::new();

    // Test with a larger length.
    alphanumeric.append_string(&mut rng, &mut output, 100);
    assert_eq!(output.len(), 100);
    assert!(output.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
#[should_panic(expected = "debug_assert! failed")]
fn test_append_string_invalid_character() {
    use rand::rngs::OsRng;

    struct InvalidAlphanumeric;
    
    impl SampleString for InvalidAlphanumeric {
        fn append_string<R: Rng + ?Sized>(
            &self,
            rng: &mut R,
            string: &mut String,
            len: usize,
        ) {
            unsafe {
                let v = string.as_mut_vec();
                v.extend(std::iter::repeat(' ').take(len)); // Spaces are not alphanumeric
            }
        }
    }

    let invalid_alphanumeric = InvalidAlphanumeric;
    let mut rng = OsRng;
    let mut output = String::new();

    // This should panic because spaces are appended.
    invalid_alphanumeric.append_string(&mut rng, &mut output, 5);
}

