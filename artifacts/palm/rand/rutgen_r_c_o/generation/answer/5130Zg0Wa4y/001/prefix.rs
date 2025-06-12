// Answer 0

#[test]
fn test_fill_bytes_empty() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: 0 };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small() {
    let mut rng = Lcg128CmDxsm64 { state: 1, increment: 1 };
    let mut dest = [0u8; 8];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_mid_size() {
    let mut rng = Lcg128CmDxsm64 { state: 2, increment: 2 };
    let mut dest = [0u8; 512];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_max_size() {
    let mut rng = Lcg128CmDxsm64 { state: 3, increment: 3 };
    let mut dest = [0u8; 1024];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_random_state() {
    let mut rng = Lcg128CmDxsm64 { state: 12345678901234567890, increment: 9876543210987654321 };
    let mut dest = [0u8; 64];
    rng.fill_bytes(&mut dest);
}

