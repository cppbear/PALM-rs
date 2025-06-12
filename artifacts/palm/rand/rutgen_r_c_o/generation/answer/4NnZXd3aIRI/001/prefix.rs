// Answer 0

#[test]
fn test_new_valid_range() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x0;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_highest_state() {
    let state = 0xffffffffffffffffffffffffffffffffffff;
    let stream = 0x0;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_middle_stream() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x3ffffffffffffffffffffffffffffffff;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_highest_stream() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x7ffffffffffffffffffffffffffffffff;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_random_state_stream() {
    let state = 0x1234567890abcdef1234567890abcdef;
    let stream = 0x9876543210abcdef9876543210abcdef;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_minimal_increment() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x1;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_maximal_increment() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x7ffffffffffffffffffffffffffffffe; // Prevents the highest bit from being set
    let rng = Lcg128CmDxsm64::new(state, stream);
}

