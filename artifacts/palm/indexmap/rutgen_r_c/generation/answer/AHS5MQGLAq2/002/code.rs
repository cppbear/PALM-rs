// Answer 0

#[test]
fn test_split_last_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let result = slice.split_last();
    assert!(result.is_none());
}

#[test]
fn test_split_last_single_element() {
    let entries = [Bucket { hash: 0, key: 1, value: 10 }];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.split_last();
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, (&1, &10));
}

#[test]
fn test_split_last_multiple_elements() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.split_last();
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, (&2, &20));
}

#[test]
fn test_split_last_multiple_elements_last_is_empty() {
    let entries: [Bucket<i32, i32>; 2] = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let (_last, rest) = slice.split_last().unwrap();
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(rest.entries.len(), 1); 
    assert_eq!(rest.get_index(0), Some((&1, &10)));
}

