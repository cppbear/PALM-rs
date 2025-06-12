// Answer 0

#[test]
fn test_thread_rng_normal() {
    let rng = thread_rng();
}

#[test]
fn test_thread_rng_edge_case() {
    let rng = thread_rng();
}

#[test]
#[should_panic]
fn test_thread_rng_panic_condition() {
    let rng = thread_rng();
}

