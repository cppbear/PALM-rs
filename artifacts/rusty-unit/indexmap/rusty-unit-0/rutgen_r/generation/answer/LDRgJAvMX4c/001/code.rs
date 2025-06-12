// Answer 0

fn test_fmt_vacant() {
    use std::fmt;

    struct Vacant<'a> {
        key: &'a str,
    }

    enum RawEntryMut<'a> {
        Vacant(Vacant<'a>),
        Occupied(&'a str),
    }

    impl<'a> fmt::Debug for Vacant<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Vacant")
                .field("key", &self.key)
                .finish()
        }
    }

    impl<'a> fmt::Debug for RawEntryMut<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut tuple = f.debug_tuple("RawEntryMut");
            match self {
                Self::Vacant(v) => tuple.field(v),
                Self::Occupied(o) => tuple.field(o),
            };
            tuple.finish()
        }
    }

    // Test case for the Vacant variant
    let vacant_entry = RawEntryMut::Vacant(Vacant { key: "test_key" });
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| vacant_entry.fmt(f));
    
    assert!(result.is_ok());
    assert!(output.contains("RawEntryMut(Vacant { key: \"test_key\" }})"));
}

fn test_fmt_occupied() {
    use std::fmt;

    struct Occupied<'a>(&'a str);
    
    enum RawEntryMut<'a> {
        Vacant(Vacant<'a>),
        Occupied(Occupied<'a>),
    }

    impl<'a> fmt::Debug for Occupied<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Occupied")
                .field("key", &self.0)
                .finish()
        }
    }

    impl<'a> fmt::Debug for RawEntryMut<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut tuple = f.debug_tuple("RawEntryMut");
            match self {
                Self::Vacant(v) => tuple.field(v),
                Self::Occupied(o) => tuple.field(o),
            };
            tuple.finish()
        }
    }

    // Test case for the Occupied variant
    let occupied_entry = RawEntryMut::Occupied(Occupied("test_key"));
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| occupied_entry.fmt(f));
    
    assert!(result.is_ok());
    assert!(output.contains("RawEntryMut(Occupied { key: \"test_key\" }))"));
}

