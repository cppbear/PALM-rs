// Answer 0

#[test]
fn test_fill_non_empty_array() {
    let mut array: [u32; 10] = Default::default();
    let mut rng = rand::thread_rng();
    array.fill(&mut rng);
}

#[test]
fn test_fill_large_array() {
    let mut array: [u64; 1000] = Default::default();
    let mut rng = rand::thread_rng();
    array.fill(&mut rng);
}

#[test]
fn test_fill_single_element_array() {
    let mut array: [i32; 1] = Default::default();
    let mut rng = rand::thread_rng();
    array.fill(&mut rng);
}

#[test]
fn test_fill_empty_slice() {
    let mut array: [bool; 0] = [];
    let mut rng = rand::thread_rng();
    array.fill(&mut rng);
}

