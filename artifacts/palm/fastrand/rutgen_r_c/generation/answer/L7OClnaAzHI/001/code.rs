// Answer 0

#[test]
fn test_shuffle_non_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice = [1, 2, 3, 4, 5];
    rng.shuffle(&mut slice);
    assert!(slice.len() == 5);
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice = [1];
    rng.shuffle(&mut slice);
    assert_eq!(slice, [1]);
}

#[should_panic(expected = "empty slice")]
#[test]
fn test_shuffle_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [i32; 0] = [];
    rng.shuffle(&mut slice);
}

#[test]
fn test_shuffle_multiple_elements() {
    let mut rng = Rng::with_seed(1);
    let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let original = slice.clone();
    rng.shuffle(&mut slice);
    assert!(slice != original);
}

#[test]
fn test_shuffle_identical_elements() {
    let mut rng = Rng::with_seed(123);
    let mut slice = [1, 1, 1, 1];
    rng.shuffle(&mut slice);
    assert_eq!(slice, [1, 1, 1, 1]);
}

