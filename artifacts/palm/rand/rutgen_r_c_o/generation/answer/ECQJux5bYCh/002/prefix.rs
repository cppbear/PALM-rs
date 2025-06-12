// Answer 0

#[test]
fn test_fill_empty_slice() {
    let mut arr: [u32; 0] = [];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

#[test]
fn test_fill_small_slice() {
    let mut arr: [u32; 1] = [0];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

#[test]
fn test_fill_medium_slice() {
    let mut arr: [u32; 5] = [0; 5];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

#[test]
fn test_fill_large_slice() {
    let mut arr: [u32; 100] = [0; 100];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

#[test]
fn test_fill_xlarge_slice() {
    let mut arr: [u32; 1000] = [0; 1000];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

#[test]
fn test_fill_extreme_slice() {
    let mut arr: [u32; 10000] = [0; 10000];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

#[test]
fn test_fill_maximum_slice() {
    let mut arr: [u32; 100000] = [0; 100000];
    let mut rng = rand_core::OsRng {};
    arr.fill(&mut rng);
}

