// Answer 0

#[test]
fn test_name_valid_capture_group() {
    struct TestMatcher<'t> {
        named_groups: std::collections::HashMap<String, usize>,
        matches: Vec<Option<Match<'t>>>,
    }

    impl<'t> TestMatcher<'t> {
        fn new() -> Self {
            Self {
                named_groups: std::collections::HashMap::new(),
                matches: Vec::new(),
            }
        }
        
        fn add_named_group(&mut self, name: &str, index: usize, matched: Option<Match<'t>>) {
            self.named_groups.insert(name.to_string(), index);
            self.matches.push(matched);
        }

        pub fn name(&self, name: &str) -> Option<&Option<Match<'t>>> {
            self.named_groups.get(name).and_then(|&i| self.matches.get(i))
        }
    }

    struct Match<'t> {
        value: &'t str,
    }

    let mut matcher = TestMatcher::new();
    matcher.add_named_group("group1", 0, Some(Match { value: "test" }));

    assert_eq!(matcher.name("group1").unwrap().as_ref().map(|m| m.value), Some("test"));
}

#[test]
fn test_name_invalid_capture_group() {
    struct TestMatcher<'t> {
        named_groups: std::collections::HashMap<String, usize>,
        matches: Vec<Option<Match<'t>>>,
    }

    impl<'t> TestMatcher<'t> {
        fn new() -> Self {
            Self {
                named_groups: std::collections::HashMap::new(),
                matches: Vec::new(),
            }
        }
        
        fn add_named_group(&mut self, name: &str, index: usize, matched: Option<Match<'t>>) {
            self.named_groups.insert(name.to_string(), index);
            self.matches.push(matched);
        }

        pub fn name(&self, name: &str) -> Option<&Option<Match<'t>>> {
            self.named_groups.get(name).and_then(|&i| self.matches.get(i))
        }
    }

    struct Match<'t> {
        value: &'t str,
    }

    let matcher = TestMatcher::new();

    assert!(matcher.name("non_existent_group").is_none());
}

#[test]
fn test_name_no_capture() {
    struct TestMatcher<'t> {
        named_groups: std::collections::HashMap<String, usize>,
        matches: Vec<Option<Match<'t>>>,
    }

    impl<'t> TestMatcher<'t> {
        fn new() -> Self {
            Self {
                named_groups: std::collections::HashMap::new(),
                matches: Vec::new(),
            }
        }
        
        fn add_named_group(&mut self, name: &str, index: usize, matched: Option<Match<'t>>) {
            self.named_groups.insert(name.to_string(), index);
            self.matches.push(matched);
        }

        pub fn name(&self, name: &str) -> Option<&Option<Match<'t>>> {
            self.named_groups.get(name).and_then(|&i| self.matches.get(i))
        }
    }

    struct Match<'t> {
        value: &'t str,
    }

    let mut matcher = TestMatcher::new();
    matcher.add_named_group("empty_group", 0, None);

    assert_eq!(matcher.name("empty_group").unwrap(), &None);
}

