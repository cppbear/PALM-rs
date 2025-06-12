// Answer 0

#[derive(Debug)]
struct NFA {
    captures: Vec<Option<String>>,
}

struct Regex {
    ro: RegexObject,
}

struct RegexObject {
    nfa: NFA,
}

impl Regex {
    pub fn capture_names(&self) -> &[Option<String>] {
        &self.ro.nfa.captures
    }
}

#[test]
fn test_capture_names_with_named_captures() {
    let regex = Regex {
        ro: RegexObject {
            nfa: NFA {
                captures: vec![Some("first".to_string()), Some("second".to_string())],
            },
        },
    };

    let names = regex.capture_names();
    assert_eq!(names.len(), 2);
    assert_eq!(names[0], Some("first".to_string()));
    assert_eq!(names[1], Some("second".to_string()));
}

#[test]
fn test_capture_names_with_unnamed_captures() {
    let regex = Regex {
        ro: RegexObject {
            nfa: NFA {
                captures: vec![None, Some("third".to_string()), None],
            },
        },
    };

    let names = regex.capture_names();
    assert_eq!(names.len(), 3);
    assert_eq!(names[0], None);
    assert_eq!(names[1], Some("third".to_string()));
    assert_eq!(names[2], None);
}

#[test]
fn test_capture_names_with_empty_captures() {
    let regex = Regex {
        ro: RegexObject {
            nfa: NFA {
                captures: vec![],
            },
        },
    };

    let names = regex.capture_names();
    assert_eq!(names.len(), 0);
}

#[test]
fn test_capture_names_with_all_none() {
    let regex = Regex {
        ro: RegexObject {
            nfa: NFA {
                captures: vec![None, None, None],
            },
        },
    };

    let names = regex.capture_names();
    assert_eq!(names.len(), 3);
    assert_eq!(names[0], None);
    assert_eq!(names[1], None);
    assert_eq!(names[2], None);
}

