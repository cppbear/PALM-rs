// Answer 0

#[test]
fn test_name_valid_case_1() {
    let text = b"Hello, World!";
    let named_groups = Arc::new(HashMap::from([
        (String::from("greeting"), 0),
    ]));
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name("greeting");
}

#[test]
fn test_name_valid_case_2() {
    let text = b"Rust programming";
    let named_groups = Arc::new(HashMap::from([
        (String::from("language"), 0),
        (String::from("activity"), 1),
    ]));
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name("language");
}

#[test]
fn test_name_empty_string() {
    let text = b"Empty name!";
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name("");
}

#[test]
fn test_name_not_found() {
    let text = b"Nonexistent group";
    let named_groups = Arc::new(HashMap::from([
        (String::from("existing"), 0),
    ]));
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name("not_existing");
}

#[test]
fn test_name_valid_case_with_multiple_groups() {
    let text = b"Matching multiple!";
    let named_groups = Arc::new(HashMap::from([
        (String::from("first"), 0),
        (String::from("second"), 1),
        (String::from("third"), 2),
    ]));
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name("second");
}

#[test]
fn test_name_edge_case_large_name() {
    let text = b"Edge case with long name";
    let long_name = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
    let named_groups = Arc::new(HashMap::from([
        (long_name.to_string(), 0),
    ]));
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name(long_name);
}

#[test]
fn test_name_valid_case_all_names() {
    let text = b"All names match";
    let named_groups = Arc::new(HashMap::from([
        (String::from("name1"), 0),
        (String::from("name2"), 1),
        (String::from("name3"), 2),
        (String::from("name4"), 3),
        (String::from("name5"), 4),
        (String::from("name6"), 5),
    ]));
    let locs = Locations(vec![]);
    let captures = Captures { text, locs, named_groups };

    let _ = captures.name("name1");
    let _ = captures.name("name2");
    let _ = captures.name("name3");
    let _ = captures.name("name4");
    let _ = captures.name("name5");
    let _ = captures.name("name6");
}

