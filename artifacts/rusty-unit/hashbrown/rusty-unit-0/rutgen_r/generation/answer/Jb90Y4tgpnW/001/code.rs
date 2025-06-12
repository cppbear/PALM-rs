// Answer 0

#[test]
fn test_get_many_unchecked_mut_valid_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    libraries.insert("Library of Congress".to_string(), 1800);
    
    unsafe {
        let [Some(a), Some(b)] = (libraries.get_many_unchecked_mut([
            "Athenæum",
            "Bodleian Library",
        ])) else { panic!() };
        assert_eq!(*a, 1807);
        assert_eq!(*b, 1602);
    }
}

#[test]
fn test_get_many_unchecked_mut_overlapping_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    
    // This should be safe because the keys don’t overlap
    unsafe {
        let result = libraries.get_many_unchecked_mut([
            "Bodleian Library",
            "Athenæum",
        ]);
        assert_eq!(result[0], Some(&mut 1602));
        assert_eq!(result[1], Some(&mut 1807));
    }
}

#[test]
fn test_get_many_unchecked_mut_missing_key() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    
    unsafe {
        let result = libraries.get_many_unchecked_mut([
            "Athenæum",
            "New York Public Library",
        ]);
        assert_eq!(result[0], Some(&mut 1807));
        assert_eq!(result[1], None);
    }
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_panic_on_overlapping_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    
    // This should cause a panic as both keys are overlapping and it violates the safety contract
    unsafe {
        let _ = libraries.get_many_unchecked_mut([
            "Athenæum",
            "Athenæum", // Overlapping key
        ]);
    }
}

