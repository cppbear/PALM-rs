// Answer 0

#[derive(Debug)]
struct NFA {
    captures: Vec<Option<String>>,
}

struct Regex {
    nfa: NFA,
}

impl Regex {
    pub fn capture_names(&self) -> &[Option<String>] {
        &self.nfa.captures
    }
}

#[test]
fn test_capture_names_empty() {
    let regex = Regex {
        nfa: NFA {
            captures: Vec::new(),
        },
    };
    assert_eq!(regex.capture_names(), &[]);
}

#[test]
fn test_capture_names_some() {
    let regex = Regex {
        nfa: NFA {
            captures: vec![Some("first".to_string()), None, Some("third".to_string())],
        },
    };
    assert_eq!(regex.capture_names(), &[Some("first".to_string()), None, Some("third".to_string())]);
}

#[test]
fn test_capture_names_none() {
    let regex = Regex {
        nfa: NFA {
            captures: vec![None, None],
        },
    };
    assert_eq!(regex.capture_names(), &[None, None]);
}

