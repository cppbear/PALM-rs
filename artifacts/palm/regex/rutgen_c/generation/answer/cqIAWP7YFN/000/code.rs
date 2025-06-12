// Answer 0

#[test]
fn test_captures_debug_fmt() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::Formatter;

    struct DummyLocs {
        slots: Vec<Option<(usize, usize)>>,
    }

    impl Locations for DummyLocs {
        fn pos(&self, i: usize) -> Option<(usize, usize)> {
            self.slots.get(i).copied().flatten()
        }
        fn len(&self) -> usize {
            self.slots.len()
        }
    }

    impl SubCapturesPosIter<'_> {
        // Simulate an iterator over the slots
        fn iter(&self) -> impl Iterator<Item = Option<(usize, usize)>> + '_ {
            self.locs.slots.iter().copied()
        }
    }

    let text = b"hello world";
    let named_groups = Arc::new(HashMap::from([
        ("greet".to_string(), 0),
        ("target".to_string(), 1),
    ]));
    let locs = Locations(DummyLocs { slots: vec![Some((0, 5)), Some((6, 11))] });
    let captures = Captures { text, locs, named_groups };

    let mut formatter = Formatter::new();
    let debug_output = format!("{:?}", CapturesDebug(&captures));

    assert!(debug_output.contains("0 = \"hello\""));
    assert!(debug_output.contains("1 = \"world\""));
    assert!(debug_output.contains("greet = \"hello\""));
    assert!(debug_output.contains("target = \"world\""));
}

#[test]
fn test_captures_debug_fmt_empty() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::Formatter;

    let text = b"";
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(DummyLocs { slots: vec![] });
    let captures = Captures { text, locs, named_groups };

    let debug_output = format!("{:?}", CapturesDebug(&captures));

    assert!(debug_output.contains("0 = \"\""));
}

#[test]
fn test_captures_debug_fmt_single() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::Formatter;

    let text = b"single";
    let named_groups = Arc::new(HashMap::from([("word".to_string(), 0)]));
    let locs = Locations(DummyLocs { slots: vec![Some((0, 6))] });
    let captures = Captures { text, locs, named_groups };

    let debug_output = format!("{:?}", CapturesDebug(&captures));

    assert!(debug_output.contains("0 = \"single\""));
    assert!(debug_output.contains("word = \"single\""));
}

