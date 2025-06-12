// Answer 0

#[test]
fn test_caps_valid_pc() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2; // Assume we have 2 slots per thread
    threads.caps = vec![None; 6]; // Preallocate for 3 threads
    
    let pc = 1; // Choosing the second thread (index 1)
    let caps = threads.caps(pc);
    
    assert_eq!(caps.len(), 2); // Should return a slice of length 2
}

#[test]
#[should_panic]
fn test_caps_out_of_bounds_pc() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2; // Assume we have 2 slots per thread
    threads.caps = vec![None; 4]; // Preallocate for 2 threads
    
    let pc = 3; // Choosing an out-of-bounds index
    threads.caps(pc); // Should panic because the index is out of bounds
}

#[test]
fn test_caps_zero_pc() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2; // Assume we have 2 slots per thread
    threads.caps = vec![None; 4]; // Preallocate for 2 threads
    
    let pc = 0; // Choosing the first thread (index 0)
    let caps = threads.caps(pc);
    
    assert_eq!(caps.len(), 2); // Should return a slice of length 2
}

