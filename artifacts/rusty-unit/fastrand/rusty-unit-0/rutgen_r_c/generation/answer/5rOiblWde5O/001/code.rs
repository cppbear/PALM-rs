// Answer 0

#[test]
fn test_choice_empty_iterator() {
    let mut rng = Rng::with_seed(42);
    let empty_vec: Vec<i32> = Vec::new();
    let result = rng.choice(empty_vec);
    assert_eq!(result, None);
}

#[test]
fn test_choice_single_element() {
    let mut rng = Rng::with_seed(42);
    let single_elem_vec = vec![10];
    let result = rng.choice(single_elem_vec);
    assert_eq!(result, Some(10));
}

#[test]
fn test_choice_multiple_elements() {
    let mut rng = Rng::with_seed(42);
    let multiple_elem_vec = vec![1, 2, 3, 4, 5];
    let result = rng.choice(multiple_elem_vec);
    assert!(result.is_some());
    assert!(result.unwrap() >= 1 && result.unwrap() <= 5);
}

#[test]
fn test_choice_large_iterator() {
    let mut rng = Rng::with_seed(42);
    let large_vec: Vec<i32> = (0..1000).collect();
    let result = rng.choice(large_vec);
    assert!(result.is_some());
    assert!(result.unwrap() >= 0 && result.unwrap() < 1000);
}

