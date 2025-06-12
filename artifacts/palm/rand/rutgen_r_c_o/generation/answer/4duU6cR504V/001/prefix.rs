// Answer 0

#[test]
fn test_lcg128xsl64_fmt_min_values() {
    let rng = Lcg128Xsl64 {
        state: 0,
        increment: 0,
    };
    let _ = fmt(&rng, &mut fmt::Formatter::new());
}

#[test]
fn test_lcg128xsl64_fmt_mid_values() {
    let rng = Lcg128Xsl64 {
        state: 1 << 63,
        increment: 1 << 63,
    };
    let _ = fmt(&rng, &mut fmt::Formatter::new());
}

#[test]
fn test_lcg128xsl64_fmt_max_values() {
    let rng = Lcg128Xsl64 {
        state: u128::MAX,
        increment: u128::MAX,
    };
    let _ = fmt(&rng, &mut fmt::Formatter::new());
}

#[test]
fn test_lcg128xsl64_fmt_random_values() {
    let rng = Lcg128Xsl64 {
        state: 123456789012345678901234567890,
        increment: 987654321098765432109876543210,
    };
    let _ = fmt(&rng, &mut fmt::Formatter::new());
}

