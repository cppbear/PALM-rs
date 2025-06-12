// Answer 0

#[test]
fn test_is_word_character_high_unicode() {
    let inputs = vec![
        '\u{0080}', // U+0080
        '\u{00A0}', // U+00A0
        '\u{2000}', // U+2000
        '\u{3000}', // U+3000
        '\u{4E00}', // U+4E00
        '\u{1F600}', // U+1F600
        '\u{10FFFF}', // U+10FFFF
    ];
    for c in inputs {
        is_word_character(c);
    }
}

#[test]
fn test_is_word_character_edge_cases() {
    let edge_cases = vec![
        '\u{FFFF}', // U+FFFF - last 2-byte character
        '\u{10100}', // U+10100 - first character in supplementary planes
        '\u{1FFFF}', // U+1FFFF - near the upper limit of 2 bytes
        '\u{10FFFF}', // U+10FFFF - maximum valid Unicode character
    ];
    for c in edge_cases {
        is_word_character(c);
    }
}

