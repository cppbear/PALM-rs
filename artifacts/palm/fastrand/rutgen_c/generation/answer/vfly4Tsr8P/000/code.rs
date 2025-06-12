// Answer 0

#[test]
fn test_f32_produces_values_in_range() {
    let mut rng = Rng::with_seed(123);
    let value = rng.f32();
    assert!(value >= 0.0 && value < 1.0);
}

#[test]
fn test_f32_deterministic_output() {
    let mut rng1 = Rng::with_seed(456);
    let value1 = rng1.f32();
    
    let mut rng2 = Rng::with_seed(456);
    let value2 = rng2.f32();
    
    assert_eq!(value1, value2);
}

#[test]
fn test_f32_with_different_seeds() {
    let mut rng1 = Rng::with_seed(789);
    let value1 = rng1.f32();
    
    let mut rng2 = Rng::with_seed(101112);
    let value2 = rng2.f32();
    
    assert!(value1 != value2); // Values should differ due to different seeds
}

