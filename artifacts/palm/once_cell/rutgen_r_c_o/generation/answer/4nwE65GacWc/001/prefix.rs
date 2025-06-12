// Answer 0

#[test]
fn test_once_ref_with_valid_references() {
    let mut l = OnceRef::new();

    {
        let y = 42;
        let mut r = OnceRef::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }
}

#[test]
#[should_panic]
fn test_once_ref_with_dangling_reference() {
    let mut l = OnceRef::new();

    {
        let y = 100;
        let mut r = OnceRef::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }

    eprintln!("uaf: {}", l.get().unwrap());
}

#[test]
fn test_once_ref_with_different_types() {
    let mut l = OnceRef::new();

    {
        let y = "Hello";
        let mut r = OnceRef::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }
}

#[test]
#[should_panic]
fn test_once_ref_after_reference_scope() {
    let mut l = OnceRef::new();

    {
        let y = 3.14;
        let mut r = OnceRef::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }

    eprintln!("uaf: {}", l.get().unwrap());
}

#[test]
fn test_once_ref_with_empty_struct() {
    #[derive(Debug)]
    struct MyStruct;

    let mut l = OnceRef::new();

    {
        let y = MyStruct;
        let mut r = OnceRef::new();
        r.set(&y).unwrap();
        core::mem::swap(&mut l, &mut r);
    }
}

