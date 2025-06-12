// Answer 0

#[test]
fn test_serialize_seq_none() {
    let formatter: &mut fmt::Formatter = &mut fmt::Formatter::new();
    let _ = formatter.serialize_seq(None);
}

#[test]
fn test_serialize_seq_some_zero() {
    let formatter: &mut fmt::Formatter = &mut fmt::Formatter::new();
    let _ = formatter.serialize_seq(Some(0));
}

#[test]
fn test_serialize_seq_some_one() {
    let formatter: &mut fmt::Formatter = &mut fmt::Formatter::new();
    let _ = formatter.serialize_seq(Some(1));
}

#[test]
fn test_serialize_seq_some_max() {
    let formatter: &mut fmt::Formatter = &mut fmt::Formatter::new();
    let _ = formatter.serialize_seq(Some(usize::MAX));
}

