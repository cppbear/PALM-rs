// Answer 0

#[test]
fn test_random_u8() {
    let value: u8 = rand::random();
    assert!(value <= 255);
}

#[test]
fn test_random_f64() {
    let value: f64 = rand::random();
    assert!(value >= 0.0);
    assert!(value <= 1.0);
}

#[test]
fn test_random_bool() {
    let value: bool = rand::random();
    assert!(value == true || value == false);
}

#[test]
fn test_random_multiple_types() {
    let mut values: Vec<u8> = (0..10).map(|_| rand::random::<u8>()).collect();
    for &value in &values {
        assert!(value <= 255);
    }
    
    let mut float_values: Vec<f64> = (0..10).map(|_| rand::random::<f64>()).collect();
    for &value in &float_values {
        assert!(value >= 0.0 && value <= 1.0);
    }

    let mut bool_values: Vec<bool> = (0..10).map(|_| rand::random::<bool>()).collect();
    for &value in &bool_values {
        assert!(value == true || value == false);
    }
}

#[test]
#[should_panic]
fn test_random_incorrect_bounds() {
    let value: f64 = rand::random();
    assert!(value < 0.0); // This should panic since value should be >= 0.0
}

