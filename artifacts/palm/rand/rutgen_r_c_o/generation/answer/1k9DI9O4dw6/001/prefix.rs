// Answer 0

#[test]
fn test_new_with_min_value() {
    let state = 0x0000_0000_0000_0001;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_middle_value() {
    let state = 0x7FFF_FFFF_FFFF_FFFF;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_max_value() {
    let state = 0xFFFF_FFFF_FFFF_FFFF;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_another_low_value() {
    let state = 0x0000_0000_0000_0002;
    let rng = Mcg128Xsl64::new(state);
}

#[test]
fn test_new_with_random_valid_value() {
    let state = 0x5A5A5A5A5A5A5A5A;
    let rng = Mcg128Xsl64::new(state);
}

