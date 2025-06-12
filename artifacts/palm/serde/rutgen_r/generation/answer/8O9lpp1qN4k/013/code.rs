// Answer 0

#[test]
fn test_unexpected_with_f32() {
    struct Content {
        value: ContentType,
    }

    enum ContentType {
        F32(f32),
    }

    struct Unexpected {
        value: UnexpectedType,
    }

    enum UnexpectedType {
        Float(f64),
    }

    impl Content {
        fn unexpected(&self) -> Unexpected {
            match self.value {
                ContentType::F32(f) => Unexpected {
                    value: UnexpectedType::Float(f as f64),
                },
            }
        }
    }

    let content = Content {
        value: ContentType::F32(3.14),
    };

    let unexpected = content.unexpected();
    match unexpected.value {
        UnexpectedType::Float(f) => assert_eq!(f, 3.14_f64),
    }
}

#[test]
fn test_unexpected_with_f32_negative() {
    struct Content {
        value: ContentType,
    }

    enum ContentType {
        F32(f32),
    }

    struct Unexpected {
        value: UnexpectedType,
    }

    enum UnexpectedType {
        Float(f64),
    }

    impl Content {
        fn unexpected(&self) -> Unexpected {
            match self.value {
                ContentType::F32(f) => Unexpected {
                    value: UnexpectedType::Float(f as f64),
                },
            }
        }
    }

    let content = Content {
        value: ContentType::F32(-2.71),
    };

    let unexpected = content.unexpected();
    match unexpected.value {
        UnexpectedType::Float(f) => assert_eq!(f, -2.71_f64),
    }
}

