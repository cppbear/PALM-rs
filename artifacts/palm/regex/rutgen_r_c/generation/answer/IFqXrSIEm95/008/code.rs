// Answer 0

#[test]
fn test_description_group_unopened() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        GroupUnopened,
    }

    let error_instance = Error {
        kind: ErrorKind::GroupUnopened,
    };
    
    assert_eq!(error_instance.description(), "unopened group");
}

#[test]
fn test_description_group_name_duplicate() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        GroupNameDuplicate,
    }

    let error_instance = Error {
        kind: ErrorKind::GroupNameDuplicate,
    };
    
    assert_eq!(error_instance.description(), "duplicate capture group name");
}

#[test]
fn test_description_nest_limit_exceeded() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        NestLimitExceeded(u32),
    }

    let error_instance = Error {
        kind: ErrorKind::NestLimitExceeded(10),
    };

    assert_eq!(error_instance.description(), "nest limit exceeded");
}

