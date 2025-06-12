// Answer 0

#[test]
fn test_choose_multiple_with_exact_capacity() {
    let mut rng = Rng::with_seed(1);
    let source = vec![1, 2, 3, 4, 5];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_choose_multiple_with_less_elements() {
    let mut rng = Rng::with_seed(1);
    let source = vec![1, 2, 3];
    let amount = 5;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    let mut rng = Rng::with_seed(1);
    let source = vec![1, 2, 3, 4, 5];
    let amount = 0;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, Vec::<i32>::new());
} 

#[test]
#[should_panic]
fn test_choose_multiple_with_high_amount() {
    let mut rng = Rng::with_seed(1);
    let source: Vec<i32> = vec![];
    let amount = 1; // This will result in a panic due to an empty source
    let _result = rng.choose_multiple(source, amount);
} 

#[test]
fn test_choose_multiple_with_one_element() {
    let mut rng = Rng::with_seed(1);
    let source = vec![42];
    let amount = 1;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![42]);
} 

#[test]
fn test_choose_multiple_with_repeated_elements() {
    let mut rng = Rng::with_seed(1);
    let source = vec![1, 1, 1, 1, 1, 1];
    let amount = 3;
    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result, vec![1, 1, 1]);
}

