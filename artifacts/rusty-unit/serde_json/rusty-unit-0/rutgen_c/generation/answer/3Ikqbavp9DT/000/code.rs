// Answer 0

#[test]
fn test_fmt_positive_integer() {
    struct TestFormatter {
        output: String,
    }
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let number = Number { n: N::PosInt(42) };
    let mut formatter = TestFormatter { output: String::new() };
    number.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "42");
}

#[test]
fn test_fmt_negative_integer() {
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let number = Number { n: N::NegInt(-42) };
    let mut formatter = TestFormatter { output: String::new() };
    number.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "-42");
}

#[test]
fn test_fmt_float() {
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let number = Number { n: N::Float(3.14) };
    let mut formatter = TestFormatter { output: String::new() };
    number.fmt(&mut formatter).unwrap();
    assert!(formatter.output.contains("3.14"));
}

