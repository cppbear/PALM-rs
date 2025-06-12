// Answer 0

#[test]
fn test_get_all_empty_map() {
    struct DummyHeaderName;

    impl AsHeaderName for DummyHeaderName {
        fn find(&self, map: &HeaderMap<HeaderValue>) -> Option<(HeaderName, usize)> {
            None
        }
    }

    let map = HeaderMap::<HeaderValue>::with_capacity(10);
    let view = map.get_all(DummyHeaderName);
    assert!(view.index.is_none());
}

#[test]
fn test_get_all_single_value() {
    struct DummyHeaderName;

    impl AsHeaderName for DummyHeaderName {
        fn find(&self, map: &HeaderMap<HeaderValue>) -> Option<(HeaderName, usize)> {
            Some((HeaderName::from_static("host"), 0))
        }
    }

    let mut map = HeaderMap::<HeaderValue>::with_capacity(10);
    map.insert(DummyHeaderName, "hello".parse().unwrap());
    let view = map.get_all(DummyHeaderName);
    assert!(view.index.is_some());
}

#[test]
fn test_get_all_multiple_values() {
    struct DummyHeaderName;

    impl AsHeaderName for DummyHeaderName {
        fn find(&self, map: &HeaderMap<HeaderValue>) -> Option<(HeaderName, usize)> {
            Some((HeaderName::from_static("host"), 0))
        }
    }

    let mut map = HeaderMap::<HeaderValue>::with_capacity(10);
    map.insert(DummyHeaderName, "hello".parse().unwrap());
    map.append(DummyHeaderName, "goodbye".parse().unwrap());

    let view = map.get_all(DummyHeaderName);
    let mut iter = view.iter();

    assert_eq!(iter.next(), Some(&"hello"));
    assert_eq!(iter.next(), Some(&"goodbye"));
    assert!(iter.next().is_none());
}

