// Answer 0

#[test]
fn test_next_unsafe_no_entries() {
    struct TestHeaderMap {
        map: HeaderMap<i32>,
        iter: IterMut<'static, i32>,
    }

    let empty_map = HeaderMap {
        mask: 0,
        indices: Box::new([]),
        entries: vec![],
        extra_values: vec![],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut empty_map as *mut _,
        entry: 0,
        cursor: None,
        lt: PhantomData,
    };

    assert_eq!(iter.next_unsafe(), None);
}

#[test]
fn test_next_unsafe_with_entries() {
    struct TestHeaderMap {
        map: HeaderMap<i32>,
        iter: IterMut<'static, i32>,
    }

    let header_name = HeaderName { inner: Repr::default() };
    
    let entry = Bucket {
        hash: HashValue::default(),
        key: header_name.clone(),
        value: 42,
        links: None,
    };

    let filled_map = HeaderMap {
        mask: 0,
        indices: Box::new([Pos::new(0)]),
        entries: vec![entry],
        extra_values: vec![],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut filled_map as *mut _,
        entry: 0,
        cursor: None,
        lt: PhantomData,
    };

    let result = iter.next_unsafe();

    assert!(result.is_some());
    let (key, value_ptr) = result.unwrap();
    assert_eq!(*value_ptr, 42);
    assert_eq!(key, &header_name);
}

#[test]
fn test_next_unsafe_with_extra_values() {
    struct TestHeaderMap {
        map: HeaderMap<i32>,
        iter: IterMut<'static, i32>,
    }

    let header_name_1 = HeaderName { inner: Repr::default() };
    let header_name_2 = HeaderName { inner: Repr::default() };
    
    let entry = Bucket {
        hash: HashValue::default(),
        key: header_name_1.clone(),
        value: 42,
        links: Some(Links { next: 1, tail: 1 }),
    };

    let extra_value = ExtraValue {
        value: 84,
        prev: Link::Entry(0),
        next: Link::Entry(1),
    };

    let filled_map = HeaderMap {
        mask: 0,
        indices: Box::new([Pos::new(0)]),
        entries: vec![entry],
        extra_values: vec![extra_value],
        danger: Danger::default(),
    };

    let mut iter = IterMut {
        map: &mut filled_map as *mut _,
        entry: 0,
        cursor: None,
        lt: PhantomData,
    };

    let result1 = iter.next_unsafe().unwrap();
    assert_eq!(*result1.1, 42);

    let result2 = iter.next_unsafe().unwrap();
    assert_eq!(*result2.1, 84);
}

