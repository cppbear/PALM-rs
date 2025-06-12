// Answer 0

#[test]
fn test_new_creates_map_access() {
    struct MockDeserializer<R> {
        is_empty: bool,
        _marker: std::marker::PhantomData<R>,
    }

    impl<R> MockDeserializer<R> {
        fn new(is_empty: bool) -> Self {
            MockDeserializer {
                is_empty,
                _marker: std::marker::PhantomData,
            }
        }
    }

    struct MapAccess<'a, R> {
        de: &'a mut MockDeserializer<R>,
        first: bool,
    }

    impl<'a, R> MapAccess<'a, R> {
        fn new(de: &'a mut MockDeserializer<R>) -> Self {
            MapAccess { de, first: true }
        }
    }

    let mut deserializer = MockDeserializer::new(false);
    let map_access = MapAccess::new(&mut deserializer);

    assert_eq!(map_access.first, true);
}

