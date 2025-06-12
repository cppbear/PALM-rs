// Answer 0

#[test]
fn test_is_cut_empty() {
    let literal = Literal::empty();
    literal.is_cut();
}

#[test]
fn test_is_cut_not_cut() {
    let literal = Literal::new(vec![97, 98, 99]); // 'abc'
    literal.is_cut();
}

#[test]
fn test_is_cut_cut() {
    let mut literal = Literal::new(vec![120, 121, 122]); // 'xyz'
    literal.cut();
    literal.is_cut();
}

#[test]
fn test_is_cut_byte_range_0() {
    let literal = Literal::new(vec![0]); // single byte 0
    literal.is_cut();
}

#[test]
fn test_is_cut_byte_range_255() {
    let literal = Literal::new(vec![255]); // single byte 255
    literal.is_cut();
}

#[test]
fn test_is_cut_multiple_bytes() {
    let literal = Literal::new(vec![100, 101, 102, 103, 104]); // 'defgh'
    literal.is_cut();
}

#[test]
fn test_is_cut_cut_multiple_bytes() {
    let mut literal = Literal::new(vec![105, 106, 107]); // 'ijk'
    literal.cut();
    literal.is_cut();
}

