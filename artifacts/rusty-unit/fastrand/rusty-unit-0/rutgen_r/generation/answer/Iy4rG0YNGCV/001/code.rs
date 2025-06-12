// Answer 0

#[test]
fn test_shuffle_empty_slice() {
    let mut slice: Vec<i32> = Vec::new();
    shuffle(&mut slice);
    assert_eq!(slice, Vec::<i32>::new());
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut slice = vec![42];
    shuffle(&mut slice);
    assert_eq!(slice, vec![42]);
}

#[test]
fn test_shuffle_two_element_slice() {
    let mut slice = vec![1, 2];
    shuffle(&mut slice);
    assert!(slice == vec![1, 2] || slice == vec![2, 1]);
}

#[test]
fn test_shuffle_three_element_slice() {
    let mut slice = vec![1, 2, 3];
    shuffle(&mut slice);
    assert!(slice.contains(&1) && slice.contains(&2) && slice.contains(&3));
    assert!(slice != vec![1, 2, 3]); // checking not necessarily in original order
}

#[test]
fn test_shuffle_large_slice() {
    let mut slice: Vec<u32> = (0..1000).collect();
    let original_slice = slice.clone();
    shuffle(&mut slice);
    assert_eq!(slice.len(), original_slice.len());
    for item in original_slice {
        assert!(slice.contains(&item));
    }
}

