// Answer 0

#[test]
fn test_map_deserializer_new() {
    use serde::de::{MapDeserializer, IntoDeserializer};
    use std::marker::PhantomData;

    struct CustomIter {
        count: usize,
    }

    impl Iterator for CustomIter {
        type Item = (String, String);

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some((format!("key{}", self.count), format!("value{}", self.count)))
            } else {
                None
            }
        }
    }

    let iter = CustomIter { count: 0 };
    let deserializer = MapDeserializer::new(iter);

    assert_eq!(deserializer.count, 0);
    assert!(deserializer.value.is_none());
}

