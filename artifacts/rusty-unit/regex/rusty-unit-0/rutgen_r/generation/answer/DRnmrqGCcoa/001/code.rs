// Answer 0

#[derive(Debug)]
enum GroupKind {
    NonCapturing(Flags),
    Capturing,
}

#[derive(Debug)]
struct Flags;

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
fn test_flags_with_capturing_group() {
    let group = Group {
        kind: GroupKind::Capturing,
    };
    assert_eq!(group.flags(), None);
}

#[test]
fn test_flags_with_non_capturing_group() {
    let flags = Flags;
    let group = Group {
        kind: GroupKind::NonCapturing(flags),
    };
    assert_eq!(group.flags(), Some(&Flags));
}

#[test]
fn test_flags_with_another_group_kind() {
    #[derive(Debug)]
    enum OtherGroupKind {
        SomethingElse,
    }
    
    let group = Group {
        kind: GroupKind::NonCapturing(Flags),
    };
    assert_eq!(group.flags(), Some(&Flags)); 
}

