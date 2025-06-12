// Answer 0

#[test]
fn test_fmt_repetition_count_unclosed() {
    use std::fmt::Write; // For write! macro
    use regex_syntax::ErrorKind;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    } 

    let mut formatter = TestFormatter { output: String::new() };
    
    // Testing ErrorKind::RepetitionCountUnclosed
    let error = ErrorKind::RepetitionCountUnclosed;
    let result = error.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "unclosed counted repetition");
}

#[test]
fn test_fmt_repetition_missing() {
    use std::fmt::Write; // For write! macro
    use regex_syntax::ErrorKind;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    
    // Testing ErrorKind::RepetitionMissing
    let error = ErrorKind::RepetitionMissing;
    let result = error.fmt(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter.output, "repetition operator missing expression");
}

