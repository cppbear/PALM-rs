// Answer 0

fn create_test_captures() -> Captures<'static> {
    let text = b"Hello, world!";
    let locs = Locations(vec![Slot(0, 5), Slot(7, 12)]);
    let named_groups = Arc::new(HashMap::new());
    Captures { text, locs, named_groups }
}

#[test]
fn test_fmt_with_valid_slot() {
    let captures = create_test_captures();
    let debug = CapturesDebug(&captures);
    let mut formatter = fmt::Formatter::new();
    debug.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_named_group() {
    let mut named_groups = HashMap::new();
    named_groups.insert("g1".to_string(), 0);
    let text = b"Example text.";
    let locs = Locations(vec![Slot(0, 7)]);
    let captures = Captures { text, locs, named_groups: Arc::new(named_groups) };
    let debug = CapturesDebug(&captures);
    let mut formatter = fmt::Formatter::new();
    debug.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_invalid_slot() {
    let captures = create_test_captures();
    let debug = CapturesDebug(&captures);
    let mut formatter = fmt::Formatter::new();
    debug.fmt(&mut formatter);
}

#[test]
fn test_fmt_zero_length_locations() {
    let text = b"";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug = CapturesDebug(&captures);
    let mut formatter = fmt::Formatter::new();
    debug.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_multiple_slots() {
    let text = b"Test string for multiple slots.";
    let locs = Locations(vec![Slot(0, 4), Slot(5, 11), Slot(12, 19)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };
    let debug = CapturesDebug(&captures);
    let mut formatter = fmt::Formatter::new();
    debug.fmt(&mut formatter);
}

