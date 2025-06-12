// Answer 0

#[derive(Debug)]
enum ClassFrame {
    Union,
    Binary,
    BinaryLHS,
    BinaryRHS,
}

impl std::fmt::Display for ClassFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
fn test_class_frame_binary_rhs_display() {
    let frame = ClassFrame::BinaryRHS;
    let mut output = String::new();
    let result = write!(&mut output, "{}", frame);
    assert!(result.is_ok());
    assert_eq!(output, "BinaryRHS");
}

