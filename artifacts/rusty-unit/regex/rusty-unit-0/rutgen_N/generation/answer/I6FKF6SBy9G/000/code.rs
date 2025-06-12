// Answer 0

#[derive(Debug)]
enum ClassFrame {
    Union,
    Binary,
    BinaryLHS,
    BinaryRHS,
}

use std::fmt;

impl fmt::Display for ClassFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            ClassFrame::Union => "Union",
            ClassFrame::Binary => "Binary",
            ClassFrame::BinaryLHS => "BinaryLHS",
            ClassFrame::BinaryRHS => "BinaryRHS",
        };
        write!(f, "{}", x)
    }
}

#[test]
fn test_class_frame_union() {
    let frame = ClassFrame::Union;
    let result = format!("{}", frame);
    assert_eq!(result, "Union");
}

#[test]
fn test_class_frame_binary() {
    let frame = ClassFrame::Binary;
    let result = format!("{}", frame);
    assert_eq!(result, "Binary");
}

#[test]
fn test_class_frame_binary_lhs() {
    let frame = ClassFrame::BinaryLHS;
    let result = format!("{}", frame);
    assert_eq!(result, "BinaryLHS");
}

#[test]
fn test_class_frame_binary_rhs() {
    let frame = ClassFrame::BinaryRHS;
    let result = format!("{}", frame);
    assert_eq!(result, "BinaryRHS");
}

