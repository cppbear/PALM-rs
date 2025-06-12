// Answer 0

#[test]
fn test_split_at_empty_slice() {
    let slice: Box<Slice<i32>> = Box::new(Slice::new());
    let index = 0;
    let (first, second) = slice.split_at(index);
}

#[test]
#[should_panic]
fn test_split_at_panic_index_equal_len() {
    let slice: Box<Slice<i32>> = Box::new(Slice::new());
    let index = 0; // len is 0
    let (first, second) = slice.split_at(index);
}

#[test]
fn test_split_at_single_element() {
    let entries = [Bucket { hash: HashValue::default(), key: 1, value: "value" }];
    let slice: Box<Slice<i32>> = Box::new(Slice::from_slice(&entries));
    let index = 1;
    let (first, second) = slice.split_at(index);
}

#[test]
fn test_split_at_index_zero() {
    let entries = [
        Bucket { hash: HashValue::default(), key: 1, value: "value1" },
        Bucket { hash: HashValue::default(), key: 2, value: "value2" },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice::from_slice(&entries));
    let index = 0;
    let (first, second) = slice.split_at(index);
}

#[test]
fn test_split_at_full_length() {
    let entries = [
        Bucket { hash: HashValue::default(), key: 1, value: "value1" },
        Bucket { hash: HashValue::default(), key: 2, value: "value2" },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice::from_slice(&entries));
    let index = 2; // len is 2
    let (first, second) = slice.split_at(index);
}

#[test]
#[should_panic]
fn test_split_at_panic_index_out_of_bounds() {
    let entries = [
        Bucket { hash: HashValue::default(), key: 1, value: "value1" },
        Bucket { hash: HashValue::default(), key: 2, value: "value2" },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice::from_slice(&entries));
    let index = 3; // len is 2
    let (first, second) = slice.split_at(index);
}

