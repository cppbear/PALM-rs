// Answer 0

#[test]
fn test_get_many_key_value_mut_single_existing_key() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    let got = libraries.get_many_key_value_mut([&"Bodleian Library"]);
}

#[test]
fn test_get_many_key_value_mut_single_missing_key() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    let got = libraries.get_many_key_value_mut([&"Nonexistent Library"]);
}

#[test]
fn test_get_many_key_value_mut_two_existing_keys() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    let got = libraries.get_many_key_value_mut([
        &"Bodleian Library",
        &"Athenæum",
    ]);
}

#[test]
fn test_get_many_key_value_mut_one_existing_one_missing() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    let got = libraries.get_many_key_value_mut([
        &"Bodleian Library",
        &"Nonexistent Library",
    ]);
}

#[test]
#[should_panic]
fn test_get_many_key_value_mut_duplicate_keys() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    let got = libraries.get_many_key_value_mut([
        &"Bodleian Library",
        &"Bodleian Library",
    ]);
}

#[test]
fn test_get_many_key_value_mut_three_keys_mixed() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Library of Congress".to_string(), 1800);
    let got = libraries.get_many_key_value_mut([
        &"Bodleian Library",
        &"Athenæum",
        &"Library of Congress",
    ]);
}

#[test]
fn test_get_many_key_value_mut_four_keys_with_one_missing() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    let got = libraries.get_many_key_value_mut([
        &"Bodleian Library",
        &"Athenæum",
        &"Nonexistent Library",
        &"Library of Congress",
    ]);
}

#[test]
#[should_panic]
fn test_get_many_key_value_mut_overlapping_keys() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    let got = libraries.get_many_key_value_mut([
        &"Bodleian Library",
        &"Herzogin-Anna-Amalia-Bibliothek",
        &"Bodleian Library",
    ]);
}

