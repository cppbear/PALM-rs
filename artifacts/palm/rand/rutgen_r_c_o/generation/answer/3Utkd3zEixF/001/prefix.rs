// Answer 0

#[test]
fn test_increasing_uniform_valid_input_1() {
    let rng = rand::thread_rng();
    let n = 1;
    let increasing_uniform = IncreasingUniform::new(rng, n);
}

#[test]
fn test_increasing_uniform_valid_input_max() {
    let rng = rand::thread_rng();
    let n = u32::MAX;
    let increasing_uniform = IncreasingUniform::new(rng, n);
}

#[test]
fn test_increasing_uniform_valid_input_mid() {
    let rng = rand::thread_rng();
    let n = u32::MAX / 2;
    let increasing_uniform = IncreasingUniform::new(rng, n);
}

