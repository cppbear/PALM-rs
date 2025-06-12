// Answer 0

#[test]
fn test_fill_bytes_empty_dst() {
    let mut rng = StepRng { v: 0, a: 0 };
    let mut dst: [u8; 0] = [];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_small_dst() {
    let mut rng = StepRng { v: 1, a: 1 };
    let mut dst: [u8; 1] = [0];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_medium_dst() {
    let mut rng = StepRng { v: 2, a: 2 };
    let mut dst: [u8; 64] = [0; 64];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_large_dst() {
    let mut rng = StepRng { v: 3, a: 3 };
    let mut dst: [u8; 256] = [0; 256];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_max_dst() {
    let mut rng = StepRng { v: 4, a: 4 };
    let mut dst: [u8; 1024] = [0; 1024];
    rng.fill_bytes(&mut dst);
}

