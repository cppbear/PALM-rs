// Answer 0

#[test]
fn test_fmt_mcg128xsl64() {
    let rng = Mcg128Xsl64 { state: 0 };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", rng);
    
    let rng = Mcg128Xsl64 { state: 1 };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", rng);

    let rng = Mcg128Xsl64 { state: u128::MAX };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", rng);
    
    let rng = Mcg128Xsl64 { state: 123456789012345678901234567890 };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", rng);
    
    let rng = Mcg128Xsl64 { state: 987654321012345678901234567890 };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", rng);
}

