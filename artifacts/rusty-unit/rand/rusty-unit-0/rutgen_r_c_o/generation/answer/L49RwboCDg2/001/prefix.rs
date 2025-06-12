// Answer 0

#[test]
fn test_thread_rng_fmt_valid() {
    let rng = ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap();
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(rng)),
    };
    let mut output = String::new();
    let result = thread_rng.fmt(&mut output);
}

#[test]
fn test_thread_rng_fmt_edge_case_min() {
    let rng = ReseedingRng::new(1, OsRng).unwrap();
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(rng)),
    };
    let mut output = String::new();
    let result = thread_rng.fmt(&mut output);
}

#[test]
fn test_thread_rng_fmt_edge_case_max() {
    let rng = ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap();
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(rng)),
    };
    let mut output = String::new();
    let result = thread_rng.fmt(&mut output);
}

