// Answer 0

#[test]
fn test_get_many_mut_with_existing_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    libraries.insert("Library of Congress".to_string(), 1800);

    let [some_a, some_b] = libraries.get_many_mut([
        "Athenæum",
        "Bodleian Library",
    ]).map(|x| x.as_ref()).map(|x| x.unwrap());

    assert_eq!(*some_a, 1807);
    assert_eq!(*some_b, 1602);
}

#[test]
fn test_get_many_mut_with_missing_key() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Athenæum".to_string(), 1807);
    
    let got = libraries.get_many_mut([
        "Athenæum",
        "New York Public Library",
    ]);

    assert_eq!(got, [
        Some(&mut 1807),
        None,
    ]);
}

#[should_panic]
#[test]
fn test_get_many_mut_with_duplicate_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Athenæum".to_string(), 1807);

    let _got = libraries.get_many_mut([
        "Athenæum",
        "Athenæum",
    ]);
}

