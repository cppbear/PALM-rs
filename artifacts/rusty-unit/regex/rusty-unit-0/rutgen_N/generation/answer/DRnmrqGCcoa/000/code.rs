// Answer 0

#[derive(Debug)]
struct Flags {
    // Assume some fields here
}

#[derive(Debug)]
enum GroupKind {
    NonCapturing(Flags),
    Capturing,
}

struct Group {
    kind: GroupKind,
}

impl Group {
    pub fn flags(&self) -> Option<&Flags> {
        match self.kind {
            GroupKind::NonCapturing(ref flags) => Some(flags),
            _ => None,
        }
    }
}

#[test]
fn test_flags_non_capturing() {
    let flags = Flags { /* initialize fields */ };
    let group = Group {
        kind: GroupKind::NonCapturing(flags),
    };
    assert!(group.flags().is_some());
}

#[test]
fn test_flags_capturing() {
    let group = Group {
        kind: GroupKind::Capturing,
    };
    assert!(group.flags().is_none());
}

#[test]
fn test_flags_empty() {
    let flags = Flags { /* initialize fields */ };
    let group = Group {
        kind: GroupKind::NonCapturing(flags),
    };
    assert_eq!(group.flags().unwrap(), &flags);
}

