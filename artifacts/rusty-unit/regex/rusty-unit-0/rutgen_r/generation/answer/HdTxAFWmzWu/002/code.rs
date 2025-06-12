// Answer 0

#[derive(Debug)]
enum ClassFrame<'a> {
    Union { head: &'a str, tail: &'a [&'a str] },
    Binary { left: &'a str, right: &'a str },
    BinaryLHS { op: &'a str, rhs: &'a str },
    BinaryRHS { op: &'a str, rhs: &'a str },
}

struct Visitor;

impl Visitor {
    fn pop_class<'a>(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>> {
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
fn test_pop_class_binary_lhs() {
    let visitor = Visitor;
    let op = "&&";
    let rhs = "a > b";

    let induct = ClassFrame::BinaryLHS { op: op, rhs: rhs };

    if let Some(result) = visitor.pop_class(induct) {
        match result {
            ClassFrame::BinaryRHS { op: res_op, rhs: res_rhs } => {
                assert_eq!(res_op, op);
                assert_eq!(res_rhs, rhs);
            }
            _ => panic!("Expected BinaryRHS"),
        }
    } else {
        panic!("Expected Some(BinaryRHS), got None");
    }
}

