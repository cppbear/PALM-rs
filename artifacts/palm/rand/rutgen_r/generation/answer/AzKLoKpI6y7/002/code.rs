// Answer 0

#[test]
fn test_partial_shuffle_zero_amount() {
    use rand::Rng;
    use rand::thread_rng;

    let mut slice = [1, 2, 3, 4, 5];
    let mut rng = thread_rng();
    let amount = 0;

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);
    
    assert_eq!(shuffled, &mut []);
    assert_eq!(remaining, &mut [1, 2, 3, 4, 5]);
}

#[test]
fn test_partial_shuffle_full_amount() {
    use rand::Rng;
    use rand::thread_rng;

    let mut slice = [1, 2, 3, 4, 5];
    let mut rng = thread_rng();
    let amount = 5;

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);

    assert_eq!(shuffled.len(), 0);
    assert_eq!(remaining.len(), 5);
}

#[test]
fn test_partial_shuffle_exceeding_amount() {
    use rand::Rng;
    use rand::thread_rng;

    let mut slice = [10, 20, 30, 40, 50];
    let mut rng = thread_rng();
    let amount = 6;

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);

    assert_eq!(shuffled.len(), 5);
    assert_eq!(remaining.len(), 0);
}

#[test]
#[should_panic]
fn test_partial_shuffle_panic_split() {
    use rand::Rng;
    use rand::thread_rng;

    let mut slice: Vec<i32> = Vec::new();
    let mut rng = thread_rng();
    let amount = 1;

    let (_, _) = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_some_elements() {
    use rand::Rng;
    use rand::thread_rng;

    let mut slice = [1, 2, 3, 4, 5, 6];
    let mut rng = thread_rng();
    let amount = 3;

    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, amount);

    assert_eq!(shuffled.len(), 3);
    assert_eq!(remaining.len(), 3);
}

