// Answer 0

#[test]
fn test_name_exists() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    
    let locs = Locations(vec![]);
    let captures = Captures {
        text: "test text",
        locs,
        named_groups: Arc::new(named_groups),
    };

    let matched = captures.name("group1");
    assert!(matched.is_some());
    assert_eq!(matched.unwrap().text, "test text");
}

#[test]
fn test_name_does_not_exist() {
    let named_groups = HashMap::new();
    
    let locs = Locations(vec![]);
    let captures = Captures {
        text: "test text",
        locs,
        named_groups: Arc::new(named_groups),
    };

    let matched = captures.name("non_existing_group");
    assert!(matched.is_none());
}

#[test]
fn test_name_empty_string() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    
    let locs = Locations(vec![]);
    let captures = Captures {
        text: "test text",
        locs,
        named_groups: Arc::new(named_groups),
    };

    let matched = captures.name("");
    assert!(matched.is_none());
}

#[test]
fn test_name_with_special_characters() {
    let mut named_groups = HashMap::new();
    named_groups.insert("group$".to_string(), 0);
    
    let locs = Locations(vec![]);
    let captures = Captures {
        text: "test text",
        locs,
        named_groups: Arc::new(named_groups),
    };

    let matched = captures.name("group$");
    assert!(matched.is_some());
    assert_eq!(matched.unwrap().text, "test text");
}

#[test]
fn test_name_case_sensitivity() {
    let mut named_groups = HashMap::new();
    named_groups.insert("Group1".to_string(), 0);
    
    let locs = Locations(vec![]);
    let captures = Captures {
        text: "test text",
        locs,
        named_groups: Arc::new(named_groups),
    };

    let matched = captures.name("group1");
    assert!(matched.is_none());
}

