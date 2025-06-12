// Answer 0

#[test]
fn test_thread_rng() {
    use rand::thread_rng;
    
    let rng = thread_rng();
    assert!(rng.gen_range(0..100) >= 0);
}

#[test]
fn test_thread_rng_non_empty() {
    use rand::thread_rng;
    
    let rng = thread_rng();
    let value = rng.gen_range(0..100);
    assert!(value < 100);
}

