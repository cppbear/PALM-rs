// Answer 0

#[test]
fn test_choose_multiple_with_sufficient_elements() {
    let mut rng = Rng::with_seed(123);
    let source = (1..=10).collect::<Vec<_>>();
    let amount = 5;
    rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_excess_elements() {
    let mut rng = Rng::with_seed(456);
    let source = (1..=15).collect::<Vec<_>>();
    let amount = 10;
    rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_minimum_elements() {
    let mut rng = Rng::with_seed(789);
    let source = (1..=3).collect::<Vec<_>>();
    let amount = 3;
    rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_large_amount() {
    let mut rng = Rng::with_seed(321);
    let source = (1..=1000).collect::<Vec<_>>();
    let amount = 1000;
    rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_not_enough_elements() {
    let mut rng = Rng::with_seed(654);
    let source = (1..=5).collect::<Vec<_>>();
    let amount = 10;
    rng.choose_multiple(source, amount);
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    let mut rng = Rng::with_seed(987);
    let source = (1..=10).collect::<Vec<_>>();
    let amount = 0;
    let result = rng.choose_multiple(source, amount);
    assert_eq!(result.len(), amount);
}

#[test]
#[should_panic(expected = "empty range")]
fn test_choose_multiple_with_empty_range() {
    let mut rng = Rng::with_seed(234);
    let source: Vec<i32> = Vec::new();
    let amount = 1;
    rng.choose_multiple(source, amount);
}

