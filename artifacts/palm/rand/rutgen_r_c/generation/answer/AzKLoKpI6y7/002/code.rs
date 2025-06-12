// Answer 0

#[test]
fn test_partial_shuffle_no_elements() {
    let mut slice: Vec<u32> = Vec::new();
    let amount = 0;
    let mut rng = rand::thread_rng();

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);
    assert!(shuffled.is_empty());
    assert!(remaining.is_empty());
}

#[test]
fn test_partial_shuffle_exact_length() {
    let mut slice = vec![1, 2, 3, 4, 5];
    let amount = 5;
    let mut rng = rand::thread_rng();

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);
    assert!(shuffled.is_empty());
    assert_eq!(remaining, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_partial_shuffle_partial_amount() {
    let mut slice = vec![1, 2, 3, 4, 5];
    let amount = 3;
    let mut rng = rand::thread_rng();

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);
    assert_eq!(shuffled.len(), amount);
    assert_eq!(remaining.len(), 2);
}

#[test]
fn test_partial_shuffle_large_amount() {
    let mut slice: Vec<u32> = (1..=10).collect();
    let amount = 10;
    let mut rng = rand::thread_rng();

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);
    assert!(shuffled.is_empty());
    assert_eq!(remaining, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_partial_shuffle_no_op() {
    let mut slice = vec![1, 2, 3, 4, 5];
    let amount = 0;
    let mut rng = rand::thread_rng();

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);
    assert!(shuffled.is_empty());
    assert_eq!(remaining.len(), 5);
}

