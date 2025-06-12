// Answer 0

#[test]
fn test_get_many_key_value_unchecked_mut_valid_keys() {
    use hashbrown::HashMap;
    
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    
    unsafe {
        let got = libraries.get_many_key_value_unchecked_mut([
            "Bodleian Library",
            "Herzogin-Anna-Amalia-Bibliothek",
        ]);
        assert_eq!(
            got,
            [
                Some((&"Bodleian Library".to_string(), &mut 1602)),
                Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
            ],
        );
    }
}

#[test]
fn test_get_many_key_value_unchecked_mut_missing_key() {
    use hashbrown::HashMap;
    
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    
    unsafe {
        let got = libraries.get_many_key_value_unchecked_mut([
            "Bodleian Library",
            "Nonexistent Library",
        ]);
        assert_eq!(
            got,
            [
                Some((&"Bodleian Library".to_string(), &mut 1602)),
                None,
            ],
        );
    }
}

#[test]
#[should_panic]
fn test_get_many_key_value_unchecked_mut_overlapping_keys() {
    use hashbrown::HashMap;
    
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    
    unsafe {
        // This will cause undefined behavior if there were overlapping keys
        let _ = libraries.get_many_key_value_unchecked_mut([
            "Bodleian Library",
            "Bodleian Library",
        ]);
    }
}

#[test]
fn test_get_many_key_value_unchecked_mut_no_keys() {
    use hashbrown::HashMap;
    
    let mut libraries = HashMap::new();
    
    unsafe {
        let got = libraries.get_many_key_value_unchecked_mut([
            "Nonexistent Library",
            "Another Nonexistent Library",
        ]);
        assert_eq!(
            got,
            [
                None,
                None,
            ],
        );
    }
}

