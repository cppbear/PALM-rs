// Answer 0

#[test]
fn test_fmt_success() {
    struct TestHeader;

    impl std::fmt::Display for TestHeader {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("failed to convert header to a str")
        }
    }

    let header = TestHeader;
    let result = format!("{}", header);
    assert_eq!(result, "failed to convert header to a str");
}

#[should_panic(expected = "failed to convert header to a str")]
#[test]
fn test_fmt_panic() {
    struct PanicHeader;

    impl std::fmt::Display for PanicHeader {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            panic!("intentional panic");
        }
    }

    let panic_header = PanicHeader;
    let _ = format!("{}", panic_header);
}

