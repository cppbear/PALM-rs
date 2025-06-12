// Answer 0

#[test]
fn test_from_name_graph() {
    let result = ClassAsciiKind::from_name("graph");
    assert_eq!(result, Some(ClassAsciiKind::Graph));
}

#[test]
fn test_from_name_invalid() {
    let names = vec![
        "alnum",
        "alpha",
        "ascii",
        "blank",
        "cntrl",
        "digit",
    ];
    
    for &name in &names {
        let result = ClassAsciiKind::from_name(name);
        assert_eq!(result, None);
    }
}

