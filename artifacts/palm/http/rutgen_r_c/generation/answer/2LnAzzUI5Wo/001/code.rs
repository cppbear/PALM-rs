// Answer 0

#[test]
fn test_find_existing_entry() {
    #[derive(Clone)]
    struct TestHeaderName {
        inner: String,
    }

    impl AsRef<str> for TestHeaderName {
        fn as_ref(&self) -> &str {
            &self.inner
        }
    }

    let mut map: HeaderMap = HeaderMap {
        mask: Size::new(10),
        indices: Box::new([]),
        entries: vec![Bucket::new(TestHeaderName { inner: "Test-Header".to_string() }, HeaderValue::new("value"))],
        extra_values: vec![],
        danger: Danger::default(),
    };

    let header_name = TestHeaderName {
        inner: "Test-Header".to_string(),
    };

    assert_eq!(header_name.find(&map), Some((0, 0)));
}

#[test]
fn test_find_non_existing_entry() {
    #[derive(Clone)]
    struct TestHeaderName {
        inner: String,
    }

    impl AsRef<str> for TestHeaderName {
        fn as_ref(&self) -> &str {
            &self.inner
        }
    }

    let mut map: HeaderMap = HeaderMap {
        mask: Size::new(10),
        indices: Box::new([]),
        entries: vec![Bucket::new(TestHeaderName { inner: "Another-Header".to_string() }, HeaderValue::new("value"))],
        extra_values: vec![],
        danger: Danger::default(),
    };

    let header_name = TestHeaderName {
        inner: "Non-Existent-Header".to_string(),
    };

    assert_eq!(header_name.find(&map), None);
}

#[test]
#[should_panic]
fn test_find_on_empty_map() {
    #[derive(Clone)]
    struct TestHeaderName {
        inner: String,
    }

    impl AsRef<str> for TestHeaderName {
        fn as_ref(&self) -> &str {
            &self.inner
        }
    }

    let map: HeaderMap = HeaderMap {
        mask: Size::new(10),
        indices: Box::new([]),
        entries: vec![],
        extra_values: vec![],
        danger: Danger::default(),
    };

    let header_name = TestHeaderName {
        inner: "Any-Header".to_string(),
    };

    // This should panic as there are no entries in the HeaderMap.
    header_name.find(&map);
}

