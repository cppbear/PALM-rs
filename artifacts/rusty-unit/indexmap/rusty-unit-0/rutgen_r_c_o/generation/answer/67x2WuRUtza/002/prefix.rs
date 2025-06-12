// Answer 0

#[test]
fn test_split_first_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice::new());
    let result = slice.split_first();
}

#[test]
fn test_split_first_single_element() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 1, value: 10 }];
    let slice: Box<Slice<i32, i32>> = Box::from_raw(Box::new(Slice { entries: entries.try_into().unwrap() }));
    let result = slice.split_first();
}

