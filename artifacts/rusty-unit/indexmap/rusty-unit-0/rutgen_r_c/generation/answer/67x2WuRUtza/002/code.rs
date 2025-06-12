// Answer 0

#[test]
fn test_split_first_empty_slice() {
    let empty_slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    assert_eq!(empty_slice.split_first(), None);
}

#[test]
fn test_split_first_single_element_slice() {
    let single_item = Bucket { hash: 0, key: 1, value: 2 };
    let single_slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [single_item] });
    assert_eq!(single_slice.split_first().unwrap().0, (&1, &2));
    assert_eq!(single_slice.split_first().unwrap().1.len(), 0);
}

#[test]
fn test_split_first_multiple_elements_slice() {
    let first_item = Bucket { hash: 0, key: 1, value: 2 };
    let second_item = Bucket { hash: 1, key: 3, value: 4 };
    let multiple_slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [first_item, second_item] });
    assert_eq!(multiple_slice.split_first().unwrap().0, (&1, &2));
    assert_eq!(multiple_slice.split_first().unwrap().1.len(), 1);
}

