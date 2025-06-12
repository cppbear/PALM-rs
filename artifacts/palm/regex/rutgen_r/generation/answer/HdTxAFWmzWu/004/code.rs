// Answer 0

#[test]
fn test_pop_class_union_tail_empty() {
    struct TestStruct;

    impl TestStruct {
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

    enum ClassFrame<'a> {
        Union {
            head: &'a str,
            tail: &'a [&'a str],
        },
        Binary,
        BinaryLHS {
            op: char,
            rhs: &'a str,
        },
        BinaryRHS {
            op: char,
            rhs: &'a str,
        },
    }

    let test_struct = TestStruct;

    let induct = ClassFrame::Union {
        head: "head",
        tail: &[],
    };

    let result = test_struct.pop_class(induct);
    assert_eq!(result, None);
}

