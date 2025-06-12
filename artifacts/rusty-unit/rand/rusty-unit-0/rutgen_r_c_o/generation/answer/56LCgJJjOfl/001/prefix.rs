// Answer 0

#[test]
fn test_fill_bytes_zero_length() {
    let mut thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap())),
    };
    let mut dest: [u8; 0] = [];
    thread_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_length() {
    let mut thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap())),
    };
    let mut dest = [0u8; 10];
    thread_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_larger_length() {
    let mut thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap())),
    };
    let mut dest = [0u8; 1024];
    thread_rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_maximum_length() {
    let mut thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap())),
    };
    let mut dest = [0u8; 1024 * 64];
    thread_rng.fill_bytes(&mut dest);
}

