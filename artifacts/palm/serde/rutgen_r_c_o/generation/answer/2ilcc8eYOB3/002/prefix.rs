// Answer 0

#[test]
fn test_fmt_single_element() {
    let expected_in_seq = ExpectedInSeq(1);
    let mut formatter = std::fmt::Formatter::new();
    let _ = expected_in_seq.fmt(&mut formatter);
}

#[test]
fn test_fmt_multiple_elements() {
    let expected_in_seq = ExpectedInSeq(5);
    let mut formatter = std::fmt::Formatter::new();
    let _ = expected_in_seq.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_zero_elements() {
    let expected_in_seq = ExpectedInSeq(0);
    let mut formatter = std::fmt::Formatter::new();
    let _ = expected_in_seq.fmt(&mut formatter);
}

