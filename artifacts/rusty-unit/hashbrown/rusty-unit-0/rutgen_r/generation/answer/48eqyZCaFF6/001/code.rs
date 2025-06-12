// Answer 0

#[test]
fn test_get_many_mut_existing_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    libraries.insert("Library of Congress".to_string(), 1800);

    let [Some(a), Some(b)] = libraries.get_many_mut([
        "Athenæum",
        "Bodleian Library",
    ]) else { panic!() };

    assert_eq!(*a, 1807);
    assert_eq!(*b, 1602);
}

#[test]
fn test_get_many_mut_existing_and_missing_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Bodleian Library".to_string(), 1602);

    let got = libraries.get_many_mut([
        "Athenæum",
        "New York Public Library",
    ]);
    assert_eq!(
        got,
        [
            Some(&mut 1807),
            None
        ]
    );
}

#[should_panic]
fn test_get_many_mut_duplicate_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Athenæum".to_string(), 1807);

    let _got = libraries.get_many_mut([
        "Athenæum",
        "Athenæum",
    ]);
}

#[test]
fn test_get_many_mut_no_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();

    let got = libraries.get_many_mut([
        "Athenæum",
        "Bodleian Library",
    ]);
    assert_eq!(
        got,
        [
            None,
            None
        ]
    );
}

#[test]
fn test_get_many_mut_multiple_missing_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);

    let got = libraries.get_many_mut([
        "Athenæum",
        "New York Public Library",
    ]);
    assert_eq!(
        got,
        [
            None,
            None,
        ]
    );
}

