// Answer 0

#[test]
fn test_get_many_unchecked_mut_existing_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);

    let [some_a, some_b] = unsafe {
        libraries.get_many_unchecked_mut([
            "Athenæum",
            "Bodleian Library",
        ])
    };

    assert_eq!(some_a, Some(&mut 1807));
    assert_eq!(some_b, Some(&mut 1602));
}

#[test]
fn test_get_many_unchecked_mut_some_none() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);

    let got = unsafe {
        libraries.get_many_unchecked_mut([
            "Athenæum",
            "New York Public Library",
        ])
    };

    assert_eq!(got, [Some(&mut 1807), None]);
}

#[test]
fn test_get_many_unchecked_mut_multiple_nones() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);

    let got = unsafe {
        libraries.get_many_unchecked_mut([
            "Nonexistent Library",
            "Another Nonexistent Library",
        ])
    };

    assert_eq!(got, [None, None]);
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_overlapping_keys() {
    use hashbrown::HashMap;

    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);

    unsafe {
        // This should panic due to overlapping keys
        let _ = libraries.get_many_unchecked_mut([
            "Athenæum",
            "Athenæum", // overlapping
        ]);
    }
}

