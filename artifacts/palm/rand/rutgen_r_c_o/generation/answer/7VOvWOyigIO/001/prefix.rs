// Answer 0

#[test]
fn test_range_u8_valid() {
    let rng = &mut rand::thread_rng();
    let range = 10..100;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u8_empty() {
    let rng = &mut rand::thread_rng();
    let range = 100..100;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u16_valid() {
    let rng = &mut rand::thread_rng();
    let range = 1000..2000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u16_empty() {
    let rng = &mut rand::thread_rng();
    let range = 2000..2000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u32_valid() {
    let rng = &mut rand::thread_rng();
    let range = 10000..20000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u32_empty() {
    let rng = &mut rand::thread_rng();
    let range = 30000..30000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u64_valid() {
    let rng = &mut rand::thread_rng();
    let range = 1000000..2000000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u64_empty() {
    let rng = &mut rand::thread_rng();
    let range = 2000000..2000000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u128_valid() {
    let rng = &mut rand::thread_rng();
    let range = 10000000000..20000000000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_u128_empty() {
    let rng = &mut rand::thread_rng();
    let range = 20000000000..20000000000;
    let result = range.sample_single(rng);
}

#[test]
fn test_range_usize_valid() {
    let rng = &mut rand::thread_rng();
    let range = 0..std::usize::MAX;
    let result = range.sample_single(rng);
}

#[test]
#[should_panic]
fn test_range_usize_invalid() {
    let rng = &mut rand::thread_rng();
    let range = std::usize::MAX..std::usize::MAX;
    let result = range.sample_single(rng);
}

