// Answer 0

#[test]
fn test_choose_multiple_exact_amount() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![1, 2, 3, 4, 5];
    let amount = 3;
    
    let result = rng.choose_multiple(source.clone(), amount);
    
    assert_eq!(result.len(), amount);
    assert!(result.iter().all(|&x| source.contains(&x)));
}

#[test]
fn test_choose_multiple_less_than_amount() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![1, 2];
    let amount = 5;
    
    let result = rng.choose_multiple(source.clone(), amount);
    
    assert_eq!(result.len(), source.len());
    assert!(result.iter().all(|&x| source.contains(&x)));
}

#[test]
fn test_choose_multiple_empty_source() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![];
    let amount = 3;
    
    let result = rng.choose_multiple(source.clone(), amount);
    
    assert_eq!(result.len(), 0);
}

#[test]
fn test_choose_multiple_one_element() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![1];
    let amount = 1;
    
    let result = rng.choose_multiple(source.clone(), amount);
    
    assert_eq!(result.len(), amount);
    assert_eq!(result[0], 1);
}

#[test]
#[should_panic(expected = "empty range: {:?}", "0..0")]
fn test_choose_multiple_invalid() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<i32> = vec![];
    let amount = 0;
    
    rng.choose_multiple(source.clone(), amount);
}

