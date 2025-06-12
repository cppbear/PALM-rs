// Answer 0

#[test]
fn test_lowercase() {
    struct TestRng;

    impl TestRng {
        fn choice(&self, chars: &[u8]) -> Option<&u8> {
            Some(&chars[0]) // Always return the first character for consistent output
        }
    }

    let mut rng = TestRng;
    let result = rng.lowercase();
    assert!(result >= 'a' && result <= 'z');
}

#[test]
#[should_panic]
fn test_lowercase_should_panic() {
    struct PanicRng;

    impl PanicRng {
        fn choice(&self, _chars: &[u8]) -> Option<&u8> {
            None // Return None to trigger a panic in the function
        }
    }

    let mut rng = PanicRng;
    let _ = rng.lowercase(); // This should panic
}

