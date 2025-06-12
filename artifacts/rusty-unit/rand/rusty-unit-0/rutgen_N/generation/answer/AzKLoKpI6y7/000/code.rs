// Answer 0

#[test]
fn test_partial_shuffle_small_slice() {
    use rand::seq::SliceRandom;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(42);
    let mut slice = [1, 2, 3, 4, 5];
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 2);

    assert_eq!(shuffled.len(), 2);
    assert_eq!(remaining.len(), 3);
    assert!(remaining.iter().all(|&x| x != shuffled[0] && x != shuffled[1]));
}

#[test]
fn test_partial_shuffle_large_slice() {
    use rand::seq::SliceRandom;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(10);
    let mut slice: Vec<usize> = (0..100).collect();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 10);

    assert_eq!(shuffled.len(), 10);
    assert_eq!(remaining.len(), 90);
    assert!(remaining.iter().all(|&x| !shuffled.contains(&x)));
}

#[test]
fn test_partial_shuffle_boundary_conditions() {
    use rand::seq::SliceRandom;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(1);
    let mut slice = [1];
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 0);

    assert_eq!(shuffled.len(), 0);
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0], 1);
}

#[test]
fn test_partial_shuffle_no_elements() {
    use rand::seq::SliceRandom;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(2);
    let mut slice: Vec<u8> = Vec::new();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 0);

    assert_eq!(shuffled.len(), 0);
    assert_eq!(remaining.len(), 0);
}

