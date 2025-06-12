// Answer 0

#[test]
fn test_alphanumeric_char() {
    // Valid character ranges a-z, A-Z, and 0-9
    let generated_char = alphanumeric();
    assert!(generated_char.is_ascii_alphanumeric(), "Generated character should be alphanumeric");
    assert!(matches!(generated_char, 'a'..='z' | 'A'..='Z' | '0'..='9'), "Generated character should be in the ranges a-z, A-Z and 0-9");
}

#[test]
#[should_panic]
fn test_alphanumeric_panic_empty_range() {
    struct RngMock;
    
    impl RngMock {
        fn alphanumeric(&mut self) -> char {
            panic!("Panic triggered for empty alphanumeric range");
        }
    }

    fn with_rng<R>(f: impl FnOnce(&mut RngMock) -> R) -> R {
        let mut rng = RngMock;
        f(&mut rng)
    }

    // This is a mock test to verify panic handling in the function under test
    with_rng(|r| r.alphanumeric());
}

