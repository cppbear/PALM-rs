// Answer 0

fn test_fmt_vacant() {
    use std::fmt;

    enum RawEntryMut<'a, K> {
        Vacant(&'a K),
        Occupied(&'a K),
    }

    impl<'a, K: fmt::Debug> fmt::Debug for RawEntryMut<'a, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                RawEntryMut::Vacant(ref v) => f.debug_tuple("RawEntry").field(v).finish(),
                RawEntryMut::Occupied(ref o) => f.debug_tuple("RawEntry").field(o).finish(),
            }
        }
    }

    let key = &42; // Example key for Vacant
    let entry = RawEntryMut::Vacant(key);
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new();
        entry.fmt(formatter).unwrap();
        output = formatter.debug_str().to_string();
    }
    assert_eq!(output, format!("RawEntry({:?})", key));
}

fn test_fmt_occupied() {
    use std::fmt;

    enum RawEntryMut<'a, K> {
        Vacant(&'a K),
        Occupied(&'a K),
    }

    impl<'a, K: fmt::Debug> fmt::Debug for RawEntryMut<'a, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                RawEntryMut::Vacant(ref v) => f.debug_tuple("RawEntry").field(v).finish(),
                RawEntryMut::Occupied(ref o) => f.debug_tuple("RawEntry").field(o).finish(),
            }
        }
    }

    let key = &42; // Example key for Occupied
    let entry = RawEntryMut::Occupied(key);
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new();
        entry.fmt(formatter).unwrap();
        output = formatter.debug_str().to_string();
    }
    assert_eq!(output, format!("RawEntry({:?})", key));
}

