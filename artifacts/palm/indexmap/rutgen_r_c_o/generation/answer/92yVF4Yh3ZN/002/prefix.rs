// Answer 0

#[test]
fn test_split_last_empty_slice() {
    let slice: Box<Slice<i32>> = Box::new(Slice::new());
    let result = slice.split_last();
}

#[test]
fn test_split_last_single_element() {
    let bucket = Bucket { hash: 0, key: 1, value: "one" };
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [bucket] });
    let result = slice.split_last();
}

