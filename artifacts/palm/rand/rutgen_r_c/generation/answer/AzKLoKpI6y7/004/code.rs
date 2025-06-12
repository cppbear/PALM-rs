// Answer 0

#[test]
fn test_partial_shuffle_empty() {
    let mut slice: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 0);
    assert_eq!(shuffled.len(), 0);
    assert_eq!(remaining.len(), 0);
}

#[test]
fn test_partial_shuffle_full() {
    let mut slice: Vec<u32> = (0..10).collect();
    let mut rng = rand::thread_rng();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 10);
    assert_eq!(shuffled.len(), 0);
    assert_eq!(remaining.len(), 10);
}

#[test]
fn test_partial_shuffle_some() {
    let mut slice: Vec<u32> = (0..10).collect();
    let mut rng = rand::thread_rng();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 5);
    assert_eq!(shuffled.len(), 5);
    assert_eq!(remaining.len(), 5);
    let expected_remaining: Vec<u32> = (0..5).collect();
    assert!(remaining.iter().all(|x| expected_remaining.contains(x)));
}

#[should_panic]
fn test_partial_shuffle_panics_on_large_slice() {
    let mut slice: Vec<u32> = (0..u32::MAX as usize).collect();
    let mut rng = rand::thread_rng();
    let _ = slice.partial_shuffle(&mut rng, 1); // should panic due to self.len() not being less than u32::MAX
}

