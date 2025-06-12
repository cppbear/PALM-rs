// Answer 0

#[test]
fn test_sample_u8() {
    let mut rng = rand::thread_rng();
    let uniform_u8 = Uniform(UniformUsize);
    let result = uniform_u8.sample(&mut rng);
}

#[test]
fn test_sample_u16() {
    let mut rng = rand::thread_rng();
    let uniform_u16 = Uniform(UniformUsize);
    let result = uniform_u16.sample(&mut rng);
}

#[test]
fn test_sample_u32() {
    let mut rng = rand::thread_rng();
    let uniform_u32 = Uniform(UniformUsize);
    let result = uniform_u32.sample(&mut rng);
}

#[test]
fn test_sample_u64() {
    let mut rng = rand::thread_rng();
    let uniform_u64 = Uniform(UniformUsize);
    let result = uniform_u64.sample(&mut rng);
}

#[test]
fn test_sample_u128() {
    let mut rng = rand::thread_rng();
    let uniform_u128 = Uniform(UniformUsize);
    let result = uniform_u128.sample(&mut rng);
}

#[test]
fn test_sample_usize() {
    let mut rng = rand::thread_rng();
    let uniform_usize = Uniform(UniformUsize);
    let result = uniform_usize.sample(&mut rng);
}

