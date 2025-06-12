// Answer 0

#[test]
fn test_fmt_pos_int() {
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let number = Number { n: N::PosInt(42) };
    let mut formatter = TestFormatter;

    let result = number.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_neg_int() {
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let number = Number { n: N::NegInt(-42) };
    let mut formatter = TestFormatter;

    let result = number.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_float() {
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let number = Number { n: N::Float(3.14) };
    let mut formatter = TestFormatter;

    let result = number.fmt(&mut formatter);
    assert!(result.is_ok());
}

