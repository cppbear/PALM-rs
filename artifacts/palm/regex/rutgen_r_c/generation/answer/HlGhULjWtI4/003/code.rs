// Answer 0

fn test_fmt_binary_op_intersection() {
    use ast::{ClassSetBinaryOpKind, ClassSet};

    #[derive(Clone, Debug)]
    struct Span; // Placeholder for Span type

    #[derive(Clone, Debug)]
    struct ClassSetBinaryOp {
        pub span: Span,
        pub kind: ClassSetBinaryOpKind,
        pub lhs: Box<ClassSet>,
        pub rhs: Box<ClassSet>,
    }

    #[derive(Clone, Debug)]
    enum ClassSetBinaryOpKind {
        Intersection,
        Difference,
        SymmetricDifference,
    }

    #[derive(Clone, Debug)]
    struct ClassInduct<'a> {
        item: Option<&'a ClassSetItem>,
        binary_op: Option<&'a ClassSetBinaryOp>,
    }

    #[derive(Clone, Debug)]
    enum ClassSetItem {
        Empty(Span),
        Literal(String),
        Range(String),
        Ascii(String),
        Unicode(String),
        Perl(String),
        Bracketed(Box<ClassSet>),
        Union(ClassSet),
    }

    impl<'a> fmt::Debug for ClassInduct<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if let Some(it) = self.binary_op {
                match it.kind {
                    ClassSetBinaryOpKind::Intersection => write!(f, "BinaryOp(Intersection)"),
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        }
    }

    let span = Span;
    let lhs = Box::new(ClassSet);
    let rhs = Box::new(ClassSet);
    let binary_op = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Intersection, lhs, rhs };
    let induct = ClassInduct { item: None, binary_op: Some(&binary_op) };

    let mut buffer = String::new();
    let result = induct.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "BinaryOp(Intersection)");
}

fn test_fmt_binary_op_difference() {
    use ast::{ClassSetBinaryOpKind, ClassSet};

    #[derive(Clone, Debug)]
    struct Span; // Placeholder for Span type

    #[derive(Clone, Debug)]
    struct ClassSetBinaryOp {
        pub span: Span,
        pub kind: ClassSetBinaryOpKind,
        pub lhs: Box<ClassSet>,
        pub rhs: Box<ClassSet>,
    }

    #[derive(Clone, Debug)]
    enum ClassSetBinaryOpKind {
        Intersection,
        Difference,
        SymmetricDifference,
    }

    #[derive(Clone, Debug)]
    struct ClassInduct<'a> {
        item: Option<&'a ClassSetItem>,
        binary_op: Option<&'a ClassSetBinaryOp>,
    }

    #[derive(Clone, Debug)]
    enum ClassSetItem {
        Empty(Span),
        Literal(String),
        Range(String),
        Ascii(String),
        Unicode(String),
        Perl(String),
        Bracketed(Box<ClassSet>),
        Union(ClassSet),
    }

    impl<'a> fmt::Debug for ClassInduct<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if let Some(it) = self.binary_op {
                match it.kind {
                    ClassSetBinaryOpKind::Difference => write!(f, "BinaryOp(Difference)"),
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        }
    }

    let span = Span;
    let lhs = Box::new(ClassSet);
    let rhs = Box::new(ClassSet);
    let binary_op = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Difference, lhs, rhs };
    let induct = ClassInduct { item: None, binary_op: Some(&binary_op) };

    let mut buffer = String::new();
    let result = induct.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "BinaryOp(Difference)");
}

