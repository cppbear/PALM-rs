// Answer 0

#[test]
fn test_split_first_mut_empty() {
    let mut slice: Slice<i32, i32> = Slice::new_mut();
    let result = slice.split_first_mut();
}

#[test]
fn test_split_first_mut_single_element() {
    let mut entries = vec![Bucket { hash: 0, key: 1, value: 2 }];
    let mut slice: Slice<i32, i32> = Slice::from_mut_slice(&mut entries);
    let result = slice.split_first_mut();
}

