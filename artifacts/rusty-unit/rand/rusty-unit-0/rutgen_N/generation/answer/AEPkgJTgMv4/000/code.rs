// Answer 0

#[test]
fn test_random_u8() {
    let result: u8 = rand::random();
    assert!(result <= 255);
}

#[test]
fn test_random_f64() {
    let result: f64 = rand::random();
    assert!(result >= 0.0 && result <= 1.0);
}

#[test]
fn test_random_bool() {
    let result: bool = rand::random();
    assert!(result == true || result == false);
}

#[test]
fn test_random_vector() {
    let mut rng = rand::rng();
    let mut v: Vec<u8> = vec![0; 3];
    
    for x in v.iter_mut() {
        *x = rng.random();
    }
    
    for &x in &v {
        assert!(x <= 255);
    }
}

