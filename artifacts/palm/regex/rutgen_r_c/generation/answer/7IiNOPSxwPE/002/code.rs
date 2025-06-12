// Answer 0

fn test_expand_str() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Locations {
        // Assume we have a suitable implementation
    }

    impl Locations {
        fn pos(&self, i: usize) -> Option<(usize, usize)> {
            // Implement according to your requirement for test, this is just dummy
            Some((0, 3))
        }
    }

    struct Captures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn get(&self, i: usize) -> Option<Match<'t>> {
            self.locs.pos(i).map(|(s, e)| Match::new(self.text, s, e))
        }
        fn name(&self, name: &str) -> Option<Match<'t>> {
            self.named_groups.get(name).and_then(|&i| self.get(i))
        }
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct Match<'t> {
        text: &'t str,
        start: usize,
        end: usize,
    }

    impl<'t> Match<'t> {
        pub fn new(text: &'t str, start: usize, end: usize) -> Self {
            Match { text, start, end }
        }

        pub fn as_str(&self) -> &str {
            &self.text[self.start..self.end]
        }
    }

    #[test]
    fn test_expand_str_basic() {
        let mut dst = String::new();
        let named_groups = Arc::new(HashMap::from([("name".to_string(), 0)]));
        let captures = Captures {
            text: "Hello, world!",
            locs: Locations {},
            named_groups,
        };
        expand_str(&captures, "$name", &mut dst);
        assert_eq!(dst, "Hello, world!");
    }

    #[test]
    fn test_expand_str_with_number() {
        let mut dst = String::new();
        let named_groups = Arc::new(HashMap::new());
        let captures = Captures {
            text: "First Second",
            locs: Locations {},
            named_groups,
        };
        expand_str(&captures, "$0", &mut dst);
        assert_eq!(dst, "Fir");
    }

    #[test]
    fn test_expand_str_double_dollar() {
        let mut dst = String::new();
        let named_groups = Arc::new(HashMap::new());
        let captures = Captures {
            text: "Text Here",
            locs: Locations {},
            named_groups,
        };
        expand_str(&captures, "$$", &mut dst);
        assert_eq!(dst, "$");
    }

    #[test]
    fn test_expand_str_empty_replacement() {
        let mut dst = String::new();
        let named_groups = Arc::new(HashMap::new());
        let captures = Captures {
            text: "No match available",
            locs: Locations {},
            named_groups,
        };
        expand_str(&captures, "", &mut dst);
        assert_eq!(dst, "");
    }
}

