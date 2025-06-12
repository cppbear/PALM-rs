// Answer 0

#[test]
fn test_split_at_mut_empty_slice() {
    let mut slice = Box::new(Slice::new_mut());
    let result = slice.split_at_mut(0);
}

#[test]
fn test_split_at_mut_single_element() {
    let mut slice = Box::new(Slice::new_mut());
    slice.entries = [Bucket { hash: Default::default(), key: 1, value: "a" }];
    let result = slice.split_at_mut(1);
}

#[test]
fn test_split_at_mut_split_at_beginning() {
    let mut slice = Box::new(Slice::new_mut());
    slice.entries = [Bucket { hash: Default::default(), key: 1, value: "a" }, Bucket { hash: Default::default(), key: 2, value: "b" }];
    let result = slice.split_at_mut(0);
}

#[test]
fn test_split_at_mut_split_at_middle() {
    let mut slice = Box::new(Slice::new_mut());
    slice.entries = [
        Bucket { hash: Default::default(), key: 1, value: "a" }, 
        Bucket { hash: Default::default(), key: 2, value: "b" }
    ];
    let result = slice.split_at_mut(1);
}

#[test]
#[should_panic]
fn test_split_at_mut_panic_index_greater_than_length() {
    let mut slice = Box::new(Slice::new_mut());
    slice.entries = [Bucket { hash: Default::default(), key: 1, value: "a" }];
    let result = slice.split_at_mut(2);
}

#[test]
fn test_split_at_mut_split_at_end() {
    let mut slice = Box::new(Slice::new_mut());
    slice.entries = [
        Bucket { hash: Default::default(), key: 1, value: "a" }, 
        Bucket { hash: Default::default(), key: 2, value: "b" }, 
        Bucket { hash: Default::default(), key: 3, value: "c" }
    ];
    let result = slice.split_at_mut(3);
}

