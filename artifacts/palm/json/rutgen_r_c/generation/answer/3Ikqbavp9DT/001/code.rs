// Answer 0

#[test]
fn test_fmt_float() {
    use core::fmt::Write;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output + s;
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };

    let number = Number { n: N::Float(3.14) };
    number.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "3.14");

    let number = Number { n: N::Float(-2.718) };
    number.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "3.14-2.718");
}

#[test]
fn test_fmt_float_infinity() {
    use core::fmt::Write;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output + s;
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };

    let number = Number { n: N::Float(f64::INFINITY) };
    number.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "inf");

    let number = Number { n: N::Float(f64::NEG_INFINITY) };
    number.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "-inf");
}

#[test]
fn test_fmt_float_nan() {
    use core::fmt::Write;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output + s;
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };

    let number = Number { n: N::Float(f64::NAN) };
    if number.fmt(&mut formatter).is_ok() {
        assert!(formatter.output.is_nan());
    }
}

