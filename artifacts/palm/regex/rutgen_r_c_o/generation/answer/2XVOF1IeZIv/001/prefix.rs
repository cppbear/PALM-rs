// Answer 0

#[test]
fn test_literal_debug_cut_case_1() {
    let mut literal = Literal::new(vec![0]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_2() {
    let mut literal = Literal::new(vec![255]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_3() {
    let mut literal = Literal::new(vec![128, 255]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_4() {
    let mut literal = Literal::new(vec![32, 0, 255]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_5() {
    let mut literal = Literal::new(vec![0, 128, 255]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_6() {
    let mut literal = Literal::new(vec![1, 2, 3]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_7() {
    let mut literal = Literal::new(vec![10, 32, 127]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

#[test]
fn test_literal_debug_cut_case_8() {
    let mut literal = Literal::new(vec![100, 101, 102]);
    literal.cut();
    let _ = format!("{:?}", literal);
}

