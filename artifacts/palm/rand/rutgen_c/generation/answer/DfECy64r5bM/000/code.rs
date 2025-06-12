// Answer 0

#[test]
fn test_shuffle_empty_slice() {
    let mut slice: &mut [i32] = &mut [];
    let mut rng = rand::thread_rng();
    slice.shuffle(&mut rng);
    assert_eq!(slice, &mut []);
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut slice: &mut [i32] = &mut [1];
    let mut rng = rand::thread_rng();
    slice.shuffle(&mut rng);
    assert_eq!(slice, &mut [1]);
}

#[test]
fn test_shuffle_multiple_elements() {
    let mut slice: &mut [i32] = &mut [1, 2, 3, 4, 5];
    let mut rng = rand::thread_rng();
    slice.shuffle(&mut rng);
    // Since shuffle randomizes the order, we only check if the elements are the same
    let set: std::collections::HashSet<_> = slice.iter().cloned().collect();
    assert_eq!(set, vec![1, 2, 3, 4, 5].into_iter().collect());
}

