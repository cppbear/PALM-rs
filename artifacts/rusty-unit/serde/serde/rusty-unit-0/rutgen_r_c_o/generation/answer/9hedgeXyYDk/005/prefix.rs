// Answer 0

#[test]
fn test_visit_map_with_duplicate_start_field() {
    struct TestMapAccess {
        called_next_key: bool,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Box<dyn Error>;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.called_next_key {
                Ok(None)
            } else {
                self.called_next_key = true;
                Ok(Some(Field::Start))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(Box::new(Error::duplicate_field("start")))
        }
    }

    let visitor = RangeFromVisitor {
        expecting: "test",
        phantom: PhantomData::<i32>,
    };
    let map = TestMapAccess { called_next_key: false };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_missing_start_field() {
    struct TestMapAccess {
        called_next_key: bool,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Box<dyn Error>;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            Ok(None)
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(Box::new(Error::duplicate_field("start")))
        }
    }

    let visitor = RangeFromVisitor {
        expecting: "test",
        phantom: PhantomData::<i32>,
    };
    let map = TestMapAccess { called_next_key: false };
    let _ = visitor.visit_map(map);
} 

#[test]
fn test_visit_map_with_value_error_on_start_field() {
    struct TestMapAccess {
        called_next_key: bool,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Box<dyn Error>;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.called_next_key {
                Ok(None)
            } else {
                self.called_next_key = true;
                Ok(Some(Field::Start))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(Box::new(Error::missing_field("start")))
        }
    }

    let visitor = RangeFromVisitor {
        expecting: "test",
        phantom: PhantomData::<i32>,
    };
    let map = TestMapAccess { called_next_key: false };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_multiple_keys_on_start_field() {
    struct TestMapAccess {
        count: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = Box<dyn Error>;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.count < 2 {
                self.count += 1;
                Ok(Some(Field::Start))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(Box::new(Error::duplicate_field("start")))
        }
    }

    let visitor = RangeFromVisitor {
        expecting: "test",
        phantom: PhantomData::<i32>,
    };
    let map = TestMapAccess { count: 0 };
    let _ = visitor.visit_map(map);
}

