// Answer 0

#[derive(Debug)]
enum ClassFrame<'a> {
    Union { head: &'a str },
    Binary { op: &'a str },
    BinaryLHS { lhs: &'a str },
    BinaryRHS { rhs: &'a str },
}

struct ClassInduct<'a>(&'a str);

impl<'a> ClassInduct<'a> {
    fn from_set(value: &'a str) -> Self {
        ClassInduct(value)
    }
}

impl<'a> ClassFrame<'a> {
    fn child(&self) -> ClassInduct<'a> {
        match *self {
            ClassFrame::Union { head, .. } => ClassInduct(head),
            ClassFrame::Binary { op, .. } => ClassInduct(op),
            ClassFrame::BinaryLHS { ref lhs, .. } => {
                ClassInduct::from_set(lhs)
            }
            ClassFrame::BinaryRHS { ref rhs, .. } => {
                ClassInduct::from_set(rhs)
            }
        }
    }
}

#[test]
fn test_child_binary_lhs() {
    let lhs_input = "LEFT_HAND_SIDE";
    let frame = ClassFrame::BinaryLHS { lhs: lhs_input };
    let result = frame.child();
    assert_eq!(result.0, lhs_input);
}

#[test]
fn test_child_union() {
    let head_input = "HEAD";
    let frame = ClassFrame::Union { head: head_input };
    let result = frame.child();
    assert_eq!(result.0, head_input);
}

#[test]
fn test_child_binary() {
    let op_input = "BINARY_OP";
    let frame = ClassFrame::Binary { op: op_input };
    let result = frame.child();
    assert_eq!(result.0, op_input);
}

#[test]
fn test_child_binary_rhs() {
    let rhs_input = "RIGHT_HAND_SIDE";
    let frame = ClassFrame::BinaryRHS { rhs: rhs_input };
    let result = frame.child();
    assert_eq!(result.0, rhs_input);
}

