// Answer 0

#[test]
fn test_next_pair_with_valid_iterator() {
    use std::iter;

    struct TestItem {
        key: i32,
        value: i32,
    }

    struct TestIterator {
        current: usize,
        data: Vec<TestItem>,
    }

    impl Iterator for TestIterator {
        type Item = TestItem;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.data.len() {
                let item = self.data[self.current];
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    mod private {
        #[derive(Debug)]
        pub struct Pair;

        impl Pair {
            pub fn split(item: super::TestItem) -> (i32, i32) {
                (item.key, item.value)
            }
        }
    }

    type First<T> = i32;
    type Second<T> = i32;

    let items = vec![
        TestItem { key: 1, value: 10 },
        TestItem { key: 2, value: 20 },
    ];

    let iter = TestIterator { current: 0, data: items };
    let mut map_deserializer = MapDeserializer {
        iter: iter.iter().fuse(),
        value: None,
        count: 0,
        lifetime: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    if let Some((k, v)) = map_deserializer.next_pair() {
        assert_eq!(k, 1);
        assert_eq!(v, 10);
        assert_eq!(map_deserializer.count, 1);
    } else {
        panic!("Expected Some value but got None");
    }

    if let Some((k, v)) = map_deserializer.next_pair() {
        assert_eq!(k, 2);
        assert_eq!(v, 20);
        assert_eq!(map_deserializer.count, 2);
    } else {
        panic!("Expected Some value but got None");
    }

    assert!(map_deserializer.next_pair().is_none());
}

