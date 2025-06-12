// Answer 0

#[test]
fn test_fill_bytes_zero_length() {
    let mut rng = Lcg128Xsl64 { state: 1, increment: 1 };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_length() {
    let mut rng = Lcg128Xsl64 { state: 1, increment: 1 };
    let mut dest: [u8; 1] = [0];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_medium_length() {
    let mut rng = Lcg128Xsl64 { state: 123, increment: 456 };
    let mut dest: [u8; 10] = [0; 10];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_large_length() {
    let mut rng = Lcg128Xsl64 { state: 654321, increment: 123456 };
    let mut dest: [u8; 1024] = [0; 1024];
    rng.fill_bytes(&mut dest);
}

