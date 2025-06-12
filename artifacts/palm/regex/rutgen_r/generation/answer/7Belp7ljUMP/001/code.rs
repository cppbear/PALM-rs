// Answer 0

#[test]
fn test_expand_valid_index() {
    struct CaptureGroups {
        groups: Vec<Option<String>>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<Option<String>>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroupProvider for CaptureGroups {
        fn capture_group(&self, index: usize) -> Option<&String> {
            self.groups.get(index)?.as_ref()
        }
    }

    let capture_groups = CaptureGroups::new(vec![Some("full_match".to_string()), Some("first_group".to_string())]);
    let mut dst = String::new();
    capture_groups.expand("$0 and $1", &mut dst);
    
    assert_eq!(dst, "full_match and first_group");
}

#[test]
fn test_expand_named_group() {
    struct CaptureGroups {
        groups: std::collections::HashMap<String, String>,
    }

    impl CaptureGroups {
        fn new(groups: std::collections::HashMap<String, String>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroupProvider for CaptureGroups {
        fn capture_group(&self, name: &str) -> Option<&String> {
            self.groups.get(name)
        }
    }

    let mut named_groups = std::collections::HashMap::new();
    named_groups.insert("group1".to_string(), "named_capture".to_string());
    
    let capture_groups = CaptureGroups::new(named_groups);
    let mut dst = String::new();
    capture_groups.expand("$group1", &mut dst);
    
    assert_eq!(dst, "named_capture");
}

#[test]
fn test_expand_invalid_index() {
    struct CaptureGroups {
        groups: Vec<Option<String>>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<Option<String>>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroupProvider for CaptureGroups {
        fn capture_group(&self, index: usize) -> Option<&String> {
            self.groups.get(index)?.as_ref()
        }
    }

    let capture_groups = CaptureGroups::new(vec![Some("match".to_string())]);
    let mut dst = String::new();
    capture_groups.expand("$2", &mut dst);
    
    assert_eq!(dst, "");
}

#[test]
fn test_expand_invalid_named_group() {
    struct CaptureGroups {
        groups: std::collections::HashMap<String, String>,
    }

    impl CaptureGroups {
        fn new(groups: std::collections::HashMap<String, String>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroupProvider for CaptureGroups {
        fn capture_group(&self, name: &str) -> Option<&String> {
            self.groups.get(name)
        }
    }

    let mut named_groups = std::collections::HashMap::new();
    named_groups.insert("group1".to_string(), "valid_capture".to_string());
    
    let capture_groups = CaptureGroups::new(named_groups);
    let mut dst = String::new();
    capture_groups.expand("$invalid_group", &mut dst);
    
    assert_eq!(dst, "");
}

#[test]
fn test_expand_with_literal_dollar() {
    struct CaptureGroups {
        groups: Vec<Option<String>>,
    }

    impl CaptureGroups {
        fn new(groups: Vec<Option<String>>) -> Self {
            CaptureGroups { groups }
        }
    }

    impl CaptureGroupProvider for CaptureGroups {
        fn capture_group(&self, index: usize) -> Option<&String> {
            self.groups.get(index)?.as_ref()
        }
    }

    let capture_groups = CaptureGroups::new(vec![Some("match".to_string())]);
    let mut dst = String::new();
    capture_groups.expand("$$", &mut dst);
    
    assert_eq!(dst, "$");
}

