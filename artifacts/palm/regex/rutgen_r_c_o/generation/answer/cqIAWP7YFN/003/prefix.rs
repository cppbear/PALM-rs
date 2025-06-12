// Answer 0

#[test]
fn test_fmt_no_captures() {
    let empty_text: &[u8] = b"";
    let named_groups = Arc::new(HashMap::new());
    let locations = Locations(vec![]);
    let captures = Captures {
        text: empty_text,
        locs: locations,
        named_groups,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&captures, &mut formatter);
}

#[test]
fn test_fmt_single_capture_none() {
    let text: &[u8] = b"test";
    let named_groups = Arc::new(HashMap::new());
    let locations = Locations(vec![None]);
    let captures = Captures {
        text: text,
        locs: locations,
        named_groups,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&captures, &mut formatter);
}

#[test]
fn test_fmt_multiple_captures_none() {
    let text: &[u8] = b"example";
    let named_groups = Arc::new(HashMap::new());
    let locations = Locations(vec![None, None, None]);
    let captures = Captures {
        text: text,
        locs: locations,
        named_groups,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&captures, &mut formatter);
}

#[test]
fn test_fmt_empty_named_groups() {
    let text: &[u8] = b"data";
    let named_groups = Arc::new(HashMap::new());
    let locations = Locations(vec![None, None]);
    let captures = Captures {
        text: text,
        locs: locations,
        named_groups,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&captures, &mut formatter);
}

#[test]
fn test_fmt_no_named_groups() {
    let text: &[u8] = b"untouched";
    let named_groups = Arc::new(HashMap::new());
    let locations = Locations(vec![None; 5]);
    let captures = Captures {
        text: text,
        locs: locations,
        named_groups,
    };
    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&captures, &mut formatter);
}

