// Answer 0

#[test]
fn test_fmt_with_named_groups() {
    let text = "Hello, world";
    let mut named_groups = HashMap::new();
    named_groups.insert("greeting".to_string(), 0);
    
    let locs = Locations(vec![(0, 5), (7, 12)]);
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: Arc::new(named_groups),
    };
    
    let _ = fmt(&CapturesDebug(&captures), &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_multiple_named_groups() {
    let text = "Hello, world";
    let mut named_groups = HashMap::new();
    named_groups.insert("greeting".to_string(), 0);
    named_groups.insert("object".to_string(), 1);
    
    let locs = Locations(vec![(0, 5), (7, 12)]);
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: Arc::new(named_groups),
    };
    
    let _ = fmt(&CapturesDebug(&captures), &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_no_named_groups() {
    let text = "Hello, world";
    let named_groups = HashMap::new();

    let locs = Locations(vec![(0, 5), (7, 12)]);
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: Arc::new(named_groups),
    };
    
    let _ = fmt(&CapturesDebug(&captures), &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_empty_locations() {
    let text = "Hello";
    let named_groups = HashMap::new();

    let locs = Locations(vec![]);
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: Arc::new(named_groups),
    };
    
    let _ = fmt(&CapturesDebug(&captures), &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_five_named_groups() {
    let text = "Hello, testing multiple named groups!";
    let mut named_groups = HashMap::new();
    
    for i in 0..5 {
        named_groups.insert(format!("group{}", i), i);
    }

    let locs = Locations(vec![(0, 5), (7, 14), (16, 22), (24, 29), (31, 39)]);
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: Arc::new(named_groups),
    };
    
    let _ = fmt(&CapturesDebug(&captures), &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_out_of_bounds_slot() {
    let text = "Hello";
    let mut named_groups = HashMap::new();
    named_groups.insert("group".to_string(), 0);

    let locs = Locations(vec![(0, 5), (6, 10)]);
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: Arc::new(named_groups),
    };
    
    let _ = fmt(&CapturesDebug(&captures), &mut fmt::Formatter::new());
}

