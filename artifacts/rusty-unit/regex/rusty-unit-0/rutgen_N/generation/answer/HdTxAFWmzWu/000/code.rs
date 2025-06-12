// Answer 0

#[derive(Debug)]
enum ClassFrame<'a> {
    Union { head: &'a str, tail: &'a [&'a str] },
    Binary {},
    BinaryLHS { op: char, rhs: &'a str },
    BinaryRHS { op: char, rhs: &'a str },
}

struct TestVisitor;

impl TestVisitor {
    fn pop_class(&self, induct: ClassFrame) -> Option<ClassFrame> {
        match induct {
            ClassFrame::Union { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(ClassFrame::Union {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
            ClassFrame::Binary {..} => None,
            ClassFrame::BinaryLHS { op, rhs, .. } => {
                Some(ClassFrame::BinaryRHS {
                    op: op,
                    rhs: rhs,
                })
            }
            ClassFrame::BinaryRHS {..} => None,
        }
    }
}

#[test]
fn test_pop_class_union_non_empty() {
    let visitor = TestVisitor;
    let tail = [&"b", &"c"];
    let induct = ClassFrame::Union { head: &"a", tail: &tail };
    assert_eq!(
        visitor.pop_class(induct),
        Some(ClassFrame::Union {
            head: &"b",
            tail: &["c"]
        })
    );
}

#[test]
fn test_pop_class_union_empty() {
    let visitor = TestVisitor;
    let tail: [&str; 0] = [];
    let induct = ClassFrame::Union { head: &"a", tail: &tail };
    assert_eq!(visitor.pop_class(induct), None);
}

#[test]
fn test_pop_class_binary() {
    let visitor = TestVisitor;
    let induct = ClassFrame::Binary {};
    assert_eq!(visitor.pop_class(induct), None);
}

#[test]
fn test_pop_class_binary_lhs() {
    let visitor = TestVisitor;
    let induct = ClassFrame::BinaryLHS { op: '+', rhs: &"b" };
    assert_eq!(
        visitor.pop_class(induct),
        Some(ClassFrame::BinaryRHS {
            op: '+',
            rhs: &"b"
        })
    );
}

#[test]
fn test_pop_class_binary_rhs() {
    let visitor = TestVisitor;
    let induct = ClassFrame::BinaryRHS { op: '+', rhs: &"b" };
    assert_eq!(visitor.pop_class(induct), None);
}

