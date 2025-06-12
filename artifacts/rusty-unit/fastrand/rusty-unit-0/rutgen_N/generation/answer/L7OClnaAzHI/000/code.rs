// Answer 0

#[test]
fn test_shuffle_empty_slice() {
    let mut rng = fastrand::Rng::new();
    let mut slice: Vec<i32> = Vec::new();
    rng.shuffle(&mut slice);
    assert!(slice.is_empty());
}

#[test]
fn test_shuffle_single_element() {
    let mut rng = fastrand::Rng::new();
    let mut slice = vec![42];
    rng.shuffle(&mut slice);
    assert_eq!(slice, vec![42]);
}

#[test]
fn test_shuffle_two_elements() {
    let mut rng = fastrand::Rng::new();
    let mut slice = vec![1, 2];
    rng.shuffle(&mut slice);
    assert!(slice == vec![1, 2] || slice == vec![2, 1]);
}

#[test]
fn test_shuffle_three_elements() {
    let mut rng = fastrand::Rng::new();
    let mut slice = vec![1, 2, 3];
    rng.shuffle(&mut slice);
    assert!(slice == vec![1, 2, 3] || slice == vec![1, 3, 2] || 
            slice == vec![2, 1, 3] || slice == vec![2, 3, 1] ||
            slice == vec![3, 1, 2] || slice == vec![3, 2, 1]);
}

#[test]
fn test_shuffle_repeated_elements() {
    let mut rng = fastrand::Rng::new();
    let mut slice = vec![1, 1, 1];
    rng.shuffle(&mut slice);
    assert_eq!(slice, vec![1, 1, 1]);
}

