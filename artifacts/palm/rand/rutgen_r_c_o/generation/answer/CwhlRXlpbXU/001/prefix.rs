// Answer 0

#[test]
fn test_append_string_zero_length() {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let standard_uniform = StandardUniform;
    standard_uniform.append_string(&mut rng, &mut s, 0);
}

#[test]
fn test_append_string_one_length() {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let standard_uniform = StandardUniform;
    standard_uniform.append_string(&mut rng, &mut s, 1);
}

#[test]
fn test_append_string_large_length() {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let length = 2_usize.pow(31) - 1;
    let standard_uniform = StandardUniform;
    standard_uniform.append_string(&mut rng, &mut s, length);
}

#[test]
fn test_append_string_non_alloc_length() {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let standard_uniform = StandardUniform;
    standard_uniform.append_string(&mut rng, &mut s, 1);
}

