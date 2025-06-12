// Answer 0

#[test]
fn test_append_string_zero_length() {
    let mut rng = rand::thread_rng(); // Assuming rand crate provides thread_rng
    let mut s = String::new();
    let uniform = StandardUniform;

    // Test with zero length
    uniform.append_string(&mut rng, &mut s, 0);
    
    assert_eq!(s, "");
}

#[test]
fn test_append_string_non_zero_length() {
    let mut rng = rand::thread_rng(); // Assuming rand crate provides thread_rng
    let mut s = String::new();
    let uniform = StandardUniform;

    // Test with a positive length
    let length = 10;
    uniform.append_string(&mut rng, &mut s, length);
    
    assert_eq!(s.len(), length);
}

#[test]
fn test_append_string_large_length() {
    let mut rng = rand::thread_rng(); // Assuming rand crate provides thread_rng
    let mut s = String::new();
    let uniform = StandardUniform;

    // Test with a larger length
    let length = 1000;
    uniform.append_string(&mut rng, &mut s, length);

    assert_eq!(s.len(), length);
}

#[test]
fn test_append_string_capacity() {
    let mut rng = rand::thread_rng(); // Assuming rand crate provides thread_rng
    let mut s = String::new();
    let uniform = StandardUniform;

    // Reserve a smaller capacity initially
    s.reserve(10);
    let initial_capacity = s.capacity();

    // Test with some length
    let length = 50;
    uniform.append_string(&mut rng, &mut s, length);

    // Ensure that capacity has increased at least as expected
    assert!(s.capacity() >= initial_capacity);
    assert_eq!(s.len(), length);
}

