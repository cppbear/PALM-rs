// Answer 0

#[test]
fn test_into_slice() {
    struct MockIterator<K, V> {
        data: Vec<(K, V)>,
        idx: usize,
    }

    impl<K, V> Iterator for MockIterator<K, V> {
        type Item = (K, V);
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.idx < self.data.len() {
                let result = self.data[self.idx].clone();
                self.idx += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    struct Slice<K, V> {
        items: Vec<(K, V)>,
    }

    impl<K, V> Slice<K, V> {
        fn from_mut_slice(slice: &mut [(K, V)]) -> &'a mut Slice<K, V> {
            Slice {
                items: slice.to_vec(),
            }
        }
    }

    let data = vec![(1, "one"), (2, "two"), (3, "three")];
    let mut iterator = MockIterator { data, idx: 0 };
    
    let mut slice: &'a mut Slice<_, _> = iterator.into_slice();
    
    assert_eq!(slice.items.len(), 3);
    assert_eq!(slice.items[0], (1, "one"));
    assert_eq!(slice.items[1], (2, "two"));
    assert_eq!(slice.items[2], (3, "three"));
}

