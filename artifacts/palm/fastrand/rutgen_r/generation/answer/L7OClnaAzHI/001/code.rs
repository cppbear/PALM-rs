// Answer 0

#[test]
fn test_shuffle_non_empty_slice() {
    let mut rng = fastrand::Rng::new();
    let mut slice = [1, 2, 3, 4, 5];
    rng.shuffle(&mut slice);
    assert!(slice.iter().all(|&x| (1..=5).contains(&x)));
}

#[test]
fn test_shuffle_single_element_slice() {
    let mut rng = fastrand::Rng::new();
    let mut slice = [42];
    rng.shuffle(&mut slice);
    assert_eq!(slice, [42]);
}

#[test]
fn test_shuffle_empty_slice() {
    let mut rng = fastrand::Rng::new();
    let mut slice: Vec<i32> = Vec::new();
    rng.shuffle(&mut slice);
    assert!(slice.is_empty());
}

