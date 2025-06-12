// Answer 0

#[test]
fn test_fmt_flag_dangling_negation() {
    let error = ErrorKind::FlagDanglingNegation;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
}

#[test]
fn test_fmt_flag_dangling_negation_alternate() {
    let error = ErrorKind::FlagDanglingNegation;
    let mut output = String::new();
    let result = write!(&mut output, "Error: {}", error);
}

