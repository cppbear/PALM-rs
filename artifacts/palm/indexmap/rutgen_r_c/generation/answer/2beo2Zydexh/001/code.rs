// Answer 0

#[test]
fn test_values_on_empty_slice() {
    let slice = Slice::<i32, i32>::new();
    let mut values_iter = slice.values();
    assert!(values_iter.iter.len() == 0);
}

#[test]
fn test_values_on_single_element_slice() {
    let slice = Box::new(Slice::<i32, i32> {
        entries: [Bucket {
            hash: Default::default(),
            key: 1,
            value: 42,
        }],
    });

    let mut values_iter = slice.values();
    assert_eq!(values_iter.iter.len(), 1);
    assert_eq!(values_iter.iter.next().unwrap().value, 42);
}

#[test]
fn test_values_on_multiple_elements_slice() {
    let slice = Box::new(Slice::<i32, i32> {
        entries: [
            Bucket {
                hash: Default::default(),
                key: 1,
                value: 10,
            },
            Bucket {
                hash: Default::default(),
                key: 2,
                value: 20,
            },
            Bucket {
                hash: Default::default(),
                key: 3,
                value: 30,
            },
        ],
    });

    let mut values_iter = slice.values();
    assert_eq!(values_iter.iter.len(), 3);
    assert_eq!(values_iter.iter.next().unwrap().value, 10);
    assert_eq!(values_iter.iter.next().unwrap().value, 20);
    assert_eq!(values_iter.iter.next().unwrap().value, 30);
}

