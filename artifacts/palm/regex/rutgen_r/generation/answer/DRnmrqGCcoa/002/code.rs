// Answer 0

#[derive(Debug)]
struct Flags {
    // Example flag properties, customize as needed
    case_insensitive: bool,
    unicode: bool,
}

#[derive(Debug)]
enum GroupKind {
    NonCapturing(Flags),
    Capturing,
}

#[derive(Debug)]
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
    let flags = Flags {
        case_insensitive: true,
        unicode: false,
    };
    let group = Group {
        kind: GroupKind::NonCapturing(flags),
    };
    
    assert!(group.flags().is_some());
}

#[test]
fn test_flags_non_capturing_empty() {
    let flags = Flags {
        case_insensitive: false,
        unicode: false,
    };
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

