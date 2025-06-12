// Answer 0

#[test]
fn test_fmt() {
    struct TestLcg64Xsh32 {
        state: u64,
        increment: u64,
    }

    impl fmt::Debug for TestLcg64Xsh32 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg64Xsh32 {{}}")
        }
    }

    let rng = TestLcg64Xsh32 {
        state: 0,
        increment: 0,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", rng);
    
    assert!(result.is_ok());
    assert_eq!(output, "Lcg64Xsh32 {{}}");
}

