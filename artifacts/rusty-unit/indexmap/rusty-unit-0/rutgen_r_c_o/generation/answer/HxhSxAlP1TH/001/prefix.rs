// Answer 0

#[test]
fn test_split_at_zero() {
    let slice = Slice::from_slice(&[]);
    let result = slice.split_at(0);
}

#[test]
#[should_panic]
fn test_split_at_panic() {
    let slice = Slice::from_slice(&[]);
    let result = slice.split_at(1);
}

#[test]
fn test_split_at_one_element() {
    let entries = [Bucket { hash: 0, key: 1, value: "value1" }];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_at(1);
}

#[test]
fn test_split_at_on_two_elements() {
    let entries = [
        Bucket { hash: 0, key: 1, value: "value1" },
        Bucket { hash: 1, key: 2, value: "value2" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_at(1);
}

#[test]
fn test_split_at_full_length() {
    let entries = [
        Bucket { hash: 0, key: 1, value: "value1" },
        Bucket { hash: 1, key: 2, value: "value2" },
        Bucket { hash: 2, key: 3, value: "value3" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_at(3);
}

#[test]
#[should_panic]
fn test_split_at_out_of_bounds() {
    let entries = [
        Bucket { hash: 0, key: 1, value: "value1" },
        Bucket { hash: 1, key: 2, value: "value2" },
    ];
    let slice = Slice::from_slice(&entries);
    let result = slice.split_at(3);
}

