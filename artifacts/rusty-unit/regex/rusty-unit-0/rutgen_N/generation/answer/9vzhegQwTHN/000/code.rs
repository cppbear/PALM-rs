// Answer 0

#[test]
fn test_patterns_empty() {
    struct Dummy;

    impl Dummy {
        pub fn patterns(&self) -> &[Vec<u8>] {
            &[]
        }
    }

    let dummy = Dummy;
    let result = dummy.patterns();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_patterns_contains_no_elements() {
    struct Dummy;

    impl Dummy {
        pub fn patterns(&self) -> &[Vec<u8>] {
            &[]
        }
    }

    let dummy = Dummy;
    let result = dummy.patterns();
    assert!(result.is_empty());
}

