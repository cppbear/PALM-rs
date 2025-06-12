// Answer 0

#[test]
fn test_unexpected_u64() {
    struct Content {
        variant: ContentVariant,
    }

    enum ContentVariant {
        U64(u64),
        // other variants are omitted for brevity
    }

    struct Unexpected {
        variant: UnexpectedVariant,
    }

    enum UnexpectedVariant {
        Unsigned(u64),
        // other variants are omitted for brevity
    }

    impl Content {
        fn unexpected(&self) -> Unexpected {
            match &self.variant {
                ContentVariant::U64(n) => Unexpected { variant: UnexpectedVariant::Unsigned(*n) },
                // handling other variants is omitted for brevity
            }
        }
    }

    let content = Content { variant: ContentVariant::U64(42) };
    let unexpected = content.unexpected();

    if let UnexpectedVariant::Unsigned(value) = unexpected.variant {
        assert_eq!(value, 42);
    } else {
        panic!("Unexpected variant returned");
    }
}

