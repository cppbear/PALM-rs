// Answer 0

#[test]
fn test_named_group_matched() {
    struct TestRegex<'t> {
        named_groups: std::collections::HashMap<&'static str, usize>,
        captures: Vec<Option<Match<'t>>>,
    }

    struct Match<'t> {
        text: &'t str,
    }

    impl<'t> TestRegex<'t> {
        fn new(named_groups: std::collections::HashMap<&'static str, usize>, captures: Vec<Option<Match<'t>>>) -> Self {
            TestRegex { named_groups, captures }
        }

        fn get(&self, index: usize) -> Option<&Match<'t>> {
            self.captures.get(index).and_then(|m| m.as_ref())
        }

        fn name(&self, name: &str) -> Option<&Match<'t>> {
            self.named_groups.get(name).and_then(|&i| self.get(i))
        }
    }

    let mut named_groups = std::collections::HashMap::new();
    named_groups.insert("group1", 0);
    named_groups.insert("group2", 1);

    let captures = vec![
        Some(Match { text: "matched text" }),
        None,
    ];

    let regex = TestRegex::new(named_groups, captures);

    assert_eq!(regex.name("group1").map(|m| m.text), Some("matched text"));
    assert_eq!(regex.name("group2"), None);
}

#[test]
fn test_named_group_not_found() {
    struct TestRegex<'t> {
        named_groups: std::collections::HashMap<&'static str, usize>,
        captures: Vec<Option<Match<'t>>>,
    }

    struct Match<'t> {
        text: &'t str,
    }

    impl<'t> TestRegex<'t> {
        fn new(named_groups: std::collections::HashMap<&'static str, usize>, captures: Vec<Option<Match<'t>>>) -> Self {
            TestRegex { named_groups, captures }
        }

        fn get(&self, index: usize) -> Option<&Match<'t>> {
            self.captures.get(index).and_then(|m| m.as_ref())
        }

        fn name(&self, name: &str) -> Option<&Match<'t>> {
            self.named_groups.get(name).and_then(|&i| self.get(i))
        }
    }

    let named_groups = std::collections::HashMap::new();
    let captures = vec![
        Some(Match { text: "matched text" }),
    ];

    let regex = TestRegex::new(named_groups, captures);

    assert_eq!(regex.name("non_existent_group"), None);
}

#[test]
fn test_named_group_invalid_name() {
    struct TestRegex<'t> {
        named_groups: std::collections::HashMap<&'static str, usize>,
        captures: Vec<Option<Match<'t>>>,
    }

    struct Match<'t> {
        text: &'t str,
    }

    impl<'t> TestRegex<'t> {
        fn new(named_groups: std::collections::HashMap<&'static str, usize>, captures: Vec<Option<Match<'t>>>) -> Self {
            TestRegex { named_groups, captures }
        }

        fn get(&self, index: usize) -> Option<&Match<'t>> {
            self.captures.get(index).and_then(|m| m.as_ref())
        }

        fn name(&self, name: &str) -> Option<&Match<'t>> {
            self.named_groups.get(name).and_then(|&i| self.get(i))
        }
    }

    let mut named_groups = std::collections::HashMap::new();
    named_groups.insert("valid_group", 0);

    let captures = vec![
        Some(Match { text: "valid match" }),
    ];

    let regex = TestRegex::new(named_groups, captures);

    assert_eq!(regex.name(""), None);
} 

#[test]
#[should_panic]
fn test_named_group_access_panic() {
    struct TestRegex<'t> {
        named_groups: std::collections::HashMap<&'static str, usize>,
        captures: Vec<Option<Match<'t>>>,
    }

    struct Match<'t> {
        text: &'t str,
    }

    impl<'t> TestRegex<'t> {
        fn new(named_groups: std::collections::HashMap<&'static str, usize>, captures: Vec<Option<Match<'t>>>) -> Self {
            TestRegex { named_groups, captures }
        }

        fn get(&self, index: usize) -> Option<&Match<'t>> {
            self.captures.get(index).and_then(|m| m.as_ref())
        }

        fn name(&self, name: &str) -> Option<&Match<'t>> {
            self.named_groups.get(name).and_then(|&i| self.get(i))
        }
    }

    let mut named_groups = std::collections::HashMap::new();
    named_groups.insert("group1", 0);
    named_groups.insert("group2", 1);

    let captures = vec![
        Some(Match { text: "match1" }),
    ];

    let regex = TestRegex::new(named_groups, captures);
    let _ = regex.name("group2"); // This would panic since group2's index is out of bounds.
}

