// Answer 0

#[test]
fn test_fill_bytes_valid_length_small() {
    let mut rng = Mcg128Xsl64 { state: 1 };
    let mut buffer = [0u8; 16];
    rng.fill_bytes(&mut buffer);
}

#[test]
fn test_fill_bytes_valid_length_medium() {
    let mut rng = Mcg128Xsl64 { state: 2 };
    let mut buffer = [0u8; 256];
    rng.fill_bytes(&mut buffer);
}

#[test]
fn test_fill_bytes_valid_length_large() {
    let mut rng = Mcg128Xsl64 { state: 3 };
    let mut buffer = [0u8; 1024];
    rng.fill_bytes(&mut buffer);
}

#[test]
#[should_panic]
fn test_fill_bytes_zero_length() {
    let mut rng = Mcg128Xsl64 { state: 4 };
    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer);
}

#[test]
#[should_panic]
fn test_fill_bytes_exceeding_length() {
    let mut rng = Mcg128Xsl64 { state: 5 };
    let mut buffer = [0u8; 2048];
    rng.fill_bytes(&mut buffer);
}

