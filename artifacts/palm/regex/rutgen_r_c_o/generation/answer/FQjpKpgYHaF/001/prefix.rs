// Answer 0

#[test]
fn test_fmt_valid_char_ranges() {
    let valid_chars = vec![
        Char(0),          // U+0000 (NULL character)
        Char(65),         // U+0041 (A)
        Char(97),         // U+0061 (a)
        Char(200),        // U+00C8 (È)
        Char(1000),       // U+3E8 (Ϩ)
        Char(50000),      // U+C350 (𐰐)
        Char(1114111),    // U+10FFFF (Supplementary Planes)
    ];

    for ch in valid_chars {
        let _ = fmt(&ch, &mut fmt::Formatter::new());
    }
}

#[test]
#[should_panic]
fn test_fmt_out_of_bounds_lower() {
    let _ = fmt(&Char(u32::from(-1) as u32), &mut fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_fmt_out_of_bounds_upper() {
    let _ = fmt(&Char(1114112), &mut fmt::Formatter::new());
}

