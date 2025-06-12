// Answer 0

#[test]
fn test_shuffle_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u32] = &mut [];
    rng.shuffle(&mut slice); 
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u32] = &mut [1];
    rng.shuffle(&mut slice); 
    assert_eq!(slice, &[1]); 
}

#[test]
fn test_shuffle_two_elements_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u32] = &mut [1, 2];
    rng.shuffle(&mut slice); 
    assert!(slice == &[1, 2] || slice == &[2, 1]);
}

#[test]
fn test_shuffle_three_elements_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: &mut [u32] = &mut [1, 2, 3];
    rng.shuffle(&mut slice); 
    assert!(slice == &[1, 2, 3] || slice == &[1, 3, 2] || slice == &[2, 1, 3] || slice == &[2, 3, 1] || slice == &[3, 1, 2] || slice == &[3, 2, 1]);
}

