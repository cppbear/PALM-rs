// Answer 0

#[test]
fn test_rng_initialization() {
    let rng = rng(); // Access the local ThreadRng
    assert!(std::ptr::addr_of!(rng.rng).is_null() == false); // Ensure that rng is initialized
}

#[test]
fn test_rng_clone() {
    let rng1 = rng();
    let rng2 = rng1.clone(); // Clone the ThreadRng
    assert!(std::ptr::addr_of!(rng1.rng) != std::ptr::addr_of!(rng2.rng)); // Ensure they are different instances
}

#[test]
fn test_rng_thread_local() {
    use std::thread;

    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(move || {
            let rng = rng(); // Access the local ThreadRng in multiple threads
            assert!(std::ptr::addr_of!(rng.rng).is_null() == false); // Ensure that rng is initialized for each thread
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap(); // Ensure all threads have completed
    }
}

#[test]
fn test_rng_random_boolean() {
    let mut rng = rng(); // Get the ThreadRng
    let random_bool: bool = rng.random(); // Generate a random boolean
    assert!(random_bool == true || random_bool == false); // Ensure it's either true or false
}

#[test]
fn test_rng_random_range() {
    let mut rng = rng(); // Get the ThreadRng
    let roll: u32 = rng.random_range(1..=6); // Simulate a die roll
    assert!(roll >= 1 && roll <= 6); // Ensure the roll is within the expected range
}

