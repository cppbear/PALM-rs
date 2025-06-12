// Answer 0

#[test]
fn test_visit_content_map_ref_valid_case() {
    struct ValidVisitor;

    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let content = [
        (Content::String("key1".into()), Content::U32(42)),
        (Content::String("key2".into()), Content::Bool(true)),
    ];
    let _ = visit_content_map_ref(&content, ValidVisitor);
}

#[test]
fn test_visit_content_map_ref_empty() {
    struct ValidVisitor;

    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let content: [(Content, Content); 0] = [];
    let _ = visit_content_map_ref(&content, ValidVisitor);
}

#[test]
#[should_panic]
fn test_visit_content_map_ref_err_case() {
    struct ErrVisitor;

    impl<'de> Visitor<'de> for ErrVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Err(<M::Error as de::Error>::custom("Visitor error"))
        }
    }

    let content = [
        (Content::Char('a'), Content::F32(3.14)),
        (Content::Char('b'), Content::I64(100)),
    ];
    let _ = visit_content_map_ref(&content, ErrVisitor);
}

#[test]
fn test_visit_content_map_ref_partial_empty() {
    struct ValidVisitor;

    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let content = [
        (Content::Str("key1"), Content::None),
        (Content::Str("key2"), Content::Some(Box::new(Content::U8(5)))),
    ];
    let _ = visit_content_map_ref(&content, ValidVisitor);
}

