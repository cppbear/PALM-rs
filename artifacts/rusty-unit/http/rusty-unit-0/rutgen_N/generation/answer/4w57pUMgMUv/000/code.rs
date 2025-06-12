// Answer 0

#[test]
fn test_next_unsafe_empty() {
    struct Map {
        entries: Vec<Entry>,
        extra_values: Vec<ExtraValue>,
    }

    struct Entry {
        key: HeaderName,
        value: u32,
        links: Option<Link>,
    }

    struct ExtraValue {
        value: u32,
        next: Link,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct Cursor {
        cursor: Option<CursorState>,
        entry: usize,
        map: *mut Map,
    }

    enum CursorState {
        Head,
        Values(usize),
    }

    struct MyStruct<'a> {
        cursor: Option<CursorState>,
        entry: usize,
        map: &'a mut Map,
    }

    impl<'a> MyStruct<'a> {
        fn next_unsafe(&mut self) -> Option<(&'a HeaderName, *mut u32)> {
            use self::CursorState::*;

            if self.cursor.is_none() {
                if (self.entry + 1) >= unsafe { &*self.map }.entries.len() {
                    return None;
                }

                self.entry += 1;
                self.cursor = Some(Head);
            }

            let entry = unsafe { &mut (*self.map).entries[self.entry] };

            match self.cursor.unwrap() {
                Head => {
                    self.cursor = entry.links.as_ref().map(|l| Values(0));
                    Some((&entry.key, &mut entry.value as *mut _))
                }
                Values(idx) => {
                    let extra = unsafe { &mut (*self.map).extra_values[idx] };

                    match extra.next {
                        Link::Entry(_) => self.cursor = None,
                        Link::Extra(i) => self.cursor = Some(Values(i)),
                    }

                    Some((&entry.key, &mut extra.value as *mut _))
                }
            }
        }
    }

    let mut entries = vec![];
    let mut extra_values = vec![];

    let mut my_map = Map {
        entries,
        extra_values,
    };

    let mut my_struct = MyStruct {
        cursor: None,
        entry: 0,
        map: &mut my_map,
    };

    assert_eq!(my_struct.next_unsafe(), None);
}

#[test]
fn test_next_unsafe_with_entries() {
    struct Map {
        entries: Vec<Entry>,
        extra_values: Vec<ExtraValue>,
    }

    struct Entry {
        key: HeaderName,
        value: u32,
        links: Option<Link>,
    }

    struct ExtraValue {
        value: u32,
        next: Link,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct Cursor {
        cursor: Option<CursorState>,
        entry: usize,
        map: *mut Map,
    }

    enum CursorState {
        Head,
        Values(usize),
    }

    struct MyStruct<'a> {
        cursor: Option<CursorState>,
        entry: usize,
        map: &'a mut Map,
    }

    impl<'a> MyStruct<'a> {
        fn next_unsafe(&mut self) -> Option<(&'a HeaderName, *mut u32)> {
            use self::CursorState::*;

            if self.cursor.is_none() {
                if (self.entry + 1) >= unsafe { &*self.map }.entries.len() {
                    return None;
                }

                self.entry += 1;
                self.cursor = Some(Head);
            }

            let entry = unsafe { &mut (*self.map).entries[self.entry] };

            match self.cursor.unwrap() {
                Head => {
                    self.cursor = entry.links.as_ref().map(|l| Values(0));
                    Some((&entry.key, &mut entry.value as *mut _))
                }
                Values(idx) => {
                    let extra = unsafe { &mut (*self.map).extra_values[idx] };

                    match extra.next {
                        Link::Entry(_) => self.cursor = None,
                        Link::Extra(i) => self.cursor = Some(Values(i)),
                    }

                    Some((&entry.key, &mut extra.value as *mut _))
                }
            }
        }
    }

    struct HeaderName(String);

    let key1 = HeaderName("Key1".to_string());
    let key2 = HeaderName("Key2".to_string());

    let entries = vec![
        Entry { key: key1.clone(), value: 10, links: Some(Link::Extra(0)) },
        Entry { key: key2.clone(), value: 20, links: None },
    ];
    
    let extra_values = vec![
        ExtraValue { value: 30, next: Link::Entry(1) },
    ];

    let mut my_map = Map {
        entries,
        extra_values,
    };

    let mut my_struct = MyStruct {
        cursor: None,
        entry: 0,
        map: &mut my_map,
    };

    assert_eq!(my_struct.next_unsafe().unwrap().0, &key1);
    assert_eq!(my_struct.next_unsafe().unwrap().0, &key1); // should yield extra value
    assert_eq!(my_struct.next_unsafe().unwrap().0, &key2);
    assert_eq!(my_struct.next_unsafe(), None);
}

