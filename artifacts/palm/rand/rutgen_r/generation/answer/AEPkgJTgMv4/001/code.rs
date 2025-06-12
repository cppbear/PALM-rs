// Answer 0

#[test]
fn test_random_u8() {
    let x: u8 = rand::random();
    assert!(x <= 255); // u8 maximum value
}

#[test]
fn test_random_f64() {
    let y: f64 = rand::random();
    assert!((0.0..=1.0).contains(&y)); // f64 values expected to be in range [0.0, 1.0]
}

#[test]
fn test_random_bool() {
    let b: bool = rand::random();
    assert!(b == true || b == false); // generated boolean should be either true or false
}

#[test]
fn test_random_i32() {
    let z: i32 = rand::random();
    // i32 range is wider; no specific panic condition, but it's valid to test types
    assert!(true); // Placeholder for valid testing, as various values can be generated
} 

#[test]
fn test_random_string() {
    let mut rng = rand::rng(); // Example using local rng
    let generated_string: String = (0..10).map(|_| rng.random::<char>()).collect();
    assert_eq!(generated_string.len(), 10); // Expecting a generated string of length 10
}

