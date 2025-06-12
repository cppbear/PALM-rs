// Answer 0

#[test]
fn test_alphabetic() {
    struct TestRng;

    impl TestRng {
        fn choice<'a>(&self, choices: &'a [u8]) -> Option<&'a u8> {
            if choices.is_empty() {
                None
            } else {
                Some(&choices[0]) // always picking first for predictability
            }
        }
    }

    let mut rng = TestRng;
    let result = rng.alphabetic();
    assert!(result.is_ascii_alphabetic());
}

#[test]
#[should_panic]
fn test_alphabetic_empty_choices() {
    struct TestRngEmpty;

    impl TestRngEmpty {
        fn choice<'a>(&self, choices: &'a [u8]) -> Option<&'a u8> {
            None // simulate empty choices
        }
    }

    let mut rng_empty = TestRngEmpty;
    let _ = rng_empty.alphabetic(); // this should panic
}

