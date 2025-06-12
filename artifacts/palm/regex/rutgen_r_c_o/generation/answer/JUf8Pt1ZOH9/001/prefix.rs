// Answer 0

#[test]
fn test_cut_with_empty_literal() {
    let mut literal = Literal::empty();
    literal.cut();
}

#[test]
fn test_cut_with_non_empty_literal_one_byte() {
    let mut literal = Literal::new(vec![100]);
    literal.cut();
}

#[test]
fn test_cut_with_non_empty_literal_multiple_bytes() {
    let mut literal = Literal::new(vec![100, 150, 200]);
    literal.cut();
}

#[test]
fn test_cut_with_non_empty_literal_ten_bytes() {
    let mut literal = Literal::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    literal.cut();
}

#[test]
fn test_cut_with_boundary_values() {
    let mut literal_min = Literal::new(vec![0]);
    literal_min.cut();

    let mut literal_max = Literal::new(vec![255]);
    literal_max.cut();
}

