// Answer 0

#[test]
fn test_fmt() {
    struct Dummy {
        expecting_called: bool,
    }

    impl Dummy {
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.expecting_called = true;
            formatter.write_str("Expected value")
        }
    }

    let mut formatter = std::fmt::Formatter::new();
    let dummy = Dummy { expecting_called: false };

    let result = dummy.fmt(&mut formatter);
    assert!(result.is_ok());
    assert!(dummy.expecting_called);
}

