// Answer 0

#[test]
fn test_unexpected_with_content_i16() {
    struct Content {
        value: InnerContent,
    }

    enum InnerContent {
        I16(i16),
        // other variants omitted for brevity
    }

    struct Unexpected {
        kind: UnexpectedKind,
    }

    enum UnexpectedKind {
        Signed(i64),
        // other variants omitted for brevity
    }

    impl Content {
        fn unexpected(&self) -> Unexpected {
            match &self.value {
                InnerContent::I16(n) => Unexpected { kind: UnexpectedKind::Signed(*n as i64) },
                // other matches omitted for brevity
            }
        }
    }

    let content = Content { value: InnerContent::I16(-123) };
    if let UnexpectedKind::Signed(value) = content.unexpected().kind {
        assert_eq!(value, -123);
    }
}

#[test]
fn test_unexpected_with_content_positive_i16() {
    struct Content {
        value: InnerContent,
    }

    enum InnerContent {
        I16(i16),
        // other variants omitted for brevity
    }

    struct Unexpected {
        kind: UnexpectedKind,
    }

    enum UnexpectedKind {
        Signed(i64),
        // other variants omitted for brevity
    }

    impl Content {
        fn unexpected(&self) -> Unexpected {
            match &self.value {
                InnerContent::I16(n) => Unexpected { kind: UnexpectedKind::Signed(*n as i64) },
                // other matches omitted for brevity
            }
        }
    }

    let content = Content { value: InnerContent::I16(123) };
    if let UnexpectedKind::Signed(value) = content.unexpected().kind {
        assert_eq!(value, 123);
    }
}

