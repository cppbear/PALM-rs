// Answer 0

#[test]
fn test_shuffle_empty_slice() {
    let mut rng = fastrand::Rng::new(); 
    let mut slice: &mut [i32] = &mut []; 
    rng.shuffle(slice);
    assert_eq!(slice, &mut []);
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut rng = fastrand::Rng::new(); 
    let mut slice: &mut [i32] = &mut [42]; 
    rng.shuffle(slice);
    assert_eq!(slice, &mut [42]);
}

#[test]
#[should_panic]
fn test_shuffle_slice_with_one_element() {
    let mut rng = fastrand::Rng::new(); 
    let mut slice: &mut [i32] = &mut [1]; 
    rng.shuffle(slice);
}

