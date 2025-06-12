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

    // l now contains a dangling reference to y
    let result = l.get(); // This should trigger a dangling reference situation
    assert!(result.is_err());
}

