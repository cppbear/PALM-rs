// Answer 0

#[test]
fn test_visit_map_success() {
    struct MockMapAccess<'de> {
        entries: Vec<(&'de str, IgnoredAny)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess<'de> {
        type Error = ();

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            if self.index < self.entries.len() {
                let entry = self.entries[self.index];
                self.index += 1;
                Ok(Some((entry.0 as K, entry.1 as V)))
            } else {
                Ok(None)
            }
        }
    }

    let access = MockMapAccess {
        entries: vec![("key1", IgnoredAny), ("key2", IgnoredAny)],
        index: 0,
    };
    let visitor = InternallyTaggedUnitVisitor {
        type_name: "TestType",
        variant_name: "TestVariant",
    };
    
    assert_eq!(visitor.visit_map(access), Ok(()));
}

#[test]
fn test_visit_map_error() {
    struct MockMapAccessError {
        call_count: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccessError {
        type Error = ();

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            self.call_count += 1;

            if self.call_count == 1 {
                // Simulate an error on the second call
                return Err(());
            }
            Ok(None)
        }
    }

    let mut access = MockMapAccessError { call_count: 0 };
    let visitor = InternallyTaggedUnitVisitor {
        type_name: "TestType",
        variant_name: "TestVariant",
    };

    assert_eq!(visitor.visit_map(access), Err(()));
}

