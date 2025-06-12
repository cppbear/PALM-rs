// Answer 0

#[test]
fn test_fmt_multiple_elements_2() {
    let expected_in_seq = ExpectedInSeq(2);
    let mut formatter = std::fmt::Formatter::new();
    expected_in_seq.fmt(&mut formatter);
}

#[test]
fn test_fmt_multiple_elements_50() {
    let expected_in_seq = ExpectedInSeq(50);
    let mut formatter = std::fmt::Formatter::new();
    expected_in_seq.fmt(&mut formatter);
}

#[test]
fn test_fmt_multiple_elements_1000() {
    let expected_in_seq = ExpectedInSeq(1000);
    let mut formatter = std::fmt::Formatter::new();
    expected_in_seq.fmt(&mut formatter);
}

