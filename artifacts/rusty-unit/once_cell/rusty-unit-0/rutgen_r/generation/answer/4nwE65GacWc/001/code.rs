// Answer 0

#[test]
fn test_once_ref_dangling_reference() {
    use once_cell::race::OnceRef;

    let mut l = OnceRef::new();

    {
        let y = 2;
        let mut r = OnceRef::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }

    // This should panic because `l` should now contain a dangling reference
    let result = std::panic::catch_unwind(|| {
        eprintln!("uaf: {}", l.get().unwrap());
    });
    assert!(result.is_err());
}

#[test]
fn test_once_ref_valid_reference() {
    use once_cell::race::OnceRef;

    let y = 3;
    let mut l = OnceRef::new();
    let mut r = OnceRef::new();
    r.set(&y).unwrap();
    core::mem::swap(&mut l, &mut r);

    // l should now hold a valid reference
    assert_eq!(*l.get().unwrap(), 3);
}

