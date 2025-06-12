// Answer 0

#[test]
fn test_split_first_mut_empty() {
    let mut slice: Slice<i32, i32> = Slice::from_mut_slice(&mut []);
    assert_eq!(slice.split_first_mut(), None);
}

#[test]
fn test_split_first_mut_single_element() {
    let mut entries = [Bucket { hash: 0, key: 1, value: 10 }];
    let mut slice = Slice::from_mut_slice(&mut entries);
    if let Some((first, rest)) = slice.split_first_mut() {
        assert_eq!(*first.0, 1);
        assert_eq!(*first.1, 10);
        assert_eq!(rest.len(), 0);
    } else {
        panic!("Expected Some, got None");
    }
}

#[test]
fn test_split_first_mut_two_elements() {
    let mut entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let mut slice = Slice::from_mut_slice(&mut entries);
    if let Some((first, rest)) = slice.split_first_mut() {
        assert_eq!(*first.0, 1);
        assert_eq!(*first.1, 10);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest.get_index(0).unwrap(), (&2, &20));
    } else {
        panic!("Expected Some, got None");
    }
}

