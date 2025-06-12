// Answer 0

#[test]
fn test_visit_map_with_empty_map() {
    struct MockMap {
        entries: Vec<(Field, Idx)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            Ok(None)
        }

        fn next_value<V>(&mut self) -> Result<Idx, Self::Error> {
            Err(MockError {})
        }
    }

    let visitor = RangeFromVisitor { expecting: "test", phantom: PhantomData };
    let map = MockMap { entries: vec![], index: 0 };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_valid_start() {
    struct MockMap {
        entries: Vec<(Field, Idx)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.entries.len() {
                let entry = &self.entries[self.index];
                self.index += 1;
                Ok(Some(entry.0))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Idx, Self::Error> {
            Ok(self.entries[self.index - 1].1)
        }
    }

    let visitor = RangeFromVisitor { expecting: "test", phantom: PhantomData };
    let map = MockMap { entries: vec![(Field::Start, 42)], index: 0 };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_duplicate_start() {
    struct MockMap {
        entries: Vec<(Field, Idx)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.entries.len() {
                let entry = &self.entries[self.index];
                self.index += 1;
                Ok(Some(entry.0))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Idx, Self::Error> {
            Ok(self.entries[self.index - 1].1)
        }
    }

    let visitor = RangeFromVisitor { expecting: "test", phantom: PhantomData };
    let map = MockMap { entries: vec![(Field::Start, 42), (Field::Start, 43)], index: 0 };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_different_key() {
    struct MockMap {
        entries: Vec<(Field, Idx)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.entries.len() {
                let entry = &self.entries[self.index];
                self.index += 1;
                Ok(Some(entry.0))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Idx, Self::Error> {
            Ok(self.entries[self.index - 1].1)
        }
    }

    let visitor = RangeFromVisitor { expecting: "test", phantom: PhantomData };
    let map = MockMap { entries: vec![(Field::Start, 42), (Field::Other, 100)], index: 0 };
    let _ = visitor.visit_map(map);
}

