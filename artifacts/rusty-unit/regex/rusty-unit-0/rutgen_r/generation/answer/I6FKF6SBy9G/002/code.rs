// Answer 0

fn test_fmt_class_frame_binary_lhs() {
    use std::fmt;

    struct ClassFrame {
        kind: FrameKind,
    }

    enum FrameKind {
        Union,
        Binary,
        BinaryLHS,
        BinaryRHS,
    }

    impl fmt::Display for ClassFrame {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match self.kind {
                FrameKind::Union => "Union",
                FrameKind::Binary => "Binary",
                FrameKind::BinaryLHS => "BinaryLHS",
                FrameKind::BinaryRHS => "BinaryRHS",
            };
            write!(f, "{}", x)
        }
    }

    let frame = ClassFrame { kind: FrameKind::BinaryLHS };
    let mut output = String::new();
    let result = write!(&mut output, "{}", frame);

    assert!(result.is_ok());
    assert_eq!(output, "BinaryLHS");
}

fn test_fmt_class_frame_union() {
    use std::fmt;

    struct ClassFrame {
        kind: FrameKind,
    }

    enum FrameKind {
        Union,
        Binary,
        BinaryLHS,
        BinaryRHS,
    }

    impl fmt::Display for ClassFrame {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match self.kind {
                FrameKind::Union => "Union",
                FrameKind::Binary => "Binary",
                FrameKind::BinaryLHS => "BinaryLHS",
                FrameKind::BinaryRHS => "BinaryRHS",
            };
            write!(f, "{}", x)
        }
    }

    let frame = ClassFrame { kind: FrameKind::Union };
    let mut output = String::new();
    let result = write!(&mut output, "{}", frame);

    assert!(result.is_ok());
    assert_eq!(output, "Union");
}

fn test_fmt_class_frame_binary() {
    use std::fmt;

    struct ClassFrame {
        kind: FrameKind,
    }

    enum FrameKind {
        Union,
        Binary,
        BinaryLHS,
        BinaryRHS,
    }

    impl fmt::Display for ClassFrame {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match self.kind {
                FrameKind::Union => "Union",
                FrameKind::Binary => "Binary",
                FrameKind::BinaryLHS => "BinaryLHS",
                FrameKind::BinaryRHS => "BinaryRHS",
            };
            write!(f, "{}", x)
        }
    }

    let frame = ClassFrame { kind: FrameKind::Binary };
    let mut output = String::new();
    let result = write!(&mut output, "{}", frame);

    assert!(result.is_ok());
    assert_eq!(output, "Binary");
}

fn test_fmt_class_frame_binary_rhs() {
    use std::fmt;

    struct ClassFrame {
        kind: FrameKind,
    }

    enum FrameKind {
        Union,
        Binary,
        BinaryLHS,
        BinaryRHS,
    }

    impl fmt::Display for ClassFrame {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let x = match self.kind {
                FrameKind::Union => "Union",
                FrameKind::Binary => "Binary",
                FrameKind::BinaryLHS => "BinaryLHS",
                FrameKind::BinaryRHS => "BinaryRHS",
            };
            write!(f, "{}", x)
        }
    }

    let frame = ClassFrame { kind: FrameKind::BinaryRHS };
    let mut output = String::new();
    let result = write!(&mut output, "{}", frame);

    assert!(result.is_ok());
    assert_eq!(output, "BinaryRHS");
}

#[test]
fn run_tests() {
    test_fmt_class_frame_binary_lhs();
    test_fmt_class_frame_union();
    test_fmt_class_frame_binary();
    test_fmt_class_frame_binary_rhs();
}

