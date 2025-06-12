// Answer 0

#[test]
fn test_visit_map_success() {
    struct TestMapAccess {
        count: usize,
        max: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            if self.count < self.max {
                self.count += 1;
                Ok(Some(serde::de::IgnoredAny)) // simulating a valid key
            } else {
                Ok(None)
            }
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            if self.count <= self.max {
                Ok(Some((serde::de::IgnoredAny, serde::de::IgnoredAny))) // simulating valid entry
            } else {
                Ok(None)
            }
        }
    }

    let access = TestMapAccess { count: 0, max: 5 };
    let visitor = InternallyTaggedUnitVisitor {
        type_name: "TestType",
        variant_name: "TestVariant",
    };

    let result: Result<(), ()> = visitor.visit_map(access);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_map_empty() {
    struct EmptyMapAccess;

    impl<'de> MapAccess<'de> for EmptyMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            Ok(None)
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            Ok(None)
        }
    }

    let access = EmptyMapAccess;
    let visitor = InternallyTaggedUnitVisitor {
        type_name: "TestType",
        variant_name: "TestVariant",
    };

    let result: Result<(), ()> = visitor.visit_map(access);
    assert_eq!(result, Ok(()));
}

