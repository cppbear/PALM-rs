// Answer 0

#[test]
fn test_fmt_debug() {
    struct TestRng {
        state: u128,
        increment: u128,
    }

    impl fmt::Debug for TestRng {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg128CmDxsm64 {{}}")
        }
    }

    let rng = TestRng {
        state: 0,
        increment: 0,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", rng);
    assert!(result.is_ok());
    assert_eq!(output, "Lcg128CmDxsm64 {{}}");
}

