// Answer 0

#[test]
fn test_get_many_key_value_mut() {
    use crate::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    libraries.insert("Library of Congress".to_string(), 1800);

    let got = libraries.get_many_key_value_mut([
        "Bodleian Library",
        "Herzogin-Anna-Amalia-Bibliothek",
    ]);
    assert_eq!(
        got,
        [
            Some((&"Bodleian Library".to_string(), &mut 1602)),
            Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
        ]
    );

    let got = libraries.get_many_key_value_mut([
        "Bodleian Library",
        "NonExistent Library",
    ]);
    assert_eq!(got, [Some((&"Bodleian Library".to_string(), &mut 1602)), None]);
}

#[should_panic]
#[test]
fn test_get_many_key_value_mut_duplicate_keys() {
    use crate::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);

    let _got = libraries.get_many_key_value_mut([
        "Bodleian Library",
        "Herzogin-Anna-Amalia-Bibliothek",
        "Herzogin-Anna-Amalia-Bibliothek",
    ]);
}

