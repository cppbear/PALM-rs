// Answer 0

#[test]
fn test_prefixes_empty_literals() {
    let lits = Literals::new(vec![]);
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_single_empty_literal() {
    let lits = Literals::new(vec![vec![]]);
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_single_byte_literal() {
    let lits = Literals::new(vec![vec![b'a']]);
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_multiple_literals_one_byte() {
    let lits = Literals::new(vec![vec![b'a'], vec![b'b'], vec![b'c']]);
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_multiple_literals_with_non_ascii() {
    let lits = Literals::new(vec![vec![b'a'], vec![b'\x80']]);
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_large_number_of_literals() {
    let lits = Literals::new((0..26).map(|i| vec![i as u8]).collect());
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_teddy_avx2() {
    let lits = Literals::new((0..32).map(|i| vec![b'a']).collect());
    Matcher::prefixes(&lits);
}

#[test]
fn test_prefixes_teddy_ssse3() {
    let lits = Literals::new((0..32).map(|i| vec![b'b']).collect());
    Matcher::prefixes(&lits);
}

