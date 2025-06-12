// Answer 0

#[test]
fn test_replace_append() {
    struct TestReplacer<'t> {
        value: &'t str,
    }

    impl<'t> Replacer for TestReplacer<'t> {
        fn replace_append(&mut self, _: &Captures, dst: &mut String) {
            dst.push_str(self.value);
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            None
        }
    }

    let text_input = "Hello, world!";
    let mut dst = String::new();
    let mut replacer = TestReplacer { value: text_input };

    let locs = Locations::default(); // Assuming a default constructor exists
    let named_groups = Arc::new(HashMap::new()); // Using an empty HashMap for simplicity
    let captures = Captures {
        text: "sample",
        locs,
        named_groups,
    };

    replacer.replace_append(&captures, &mut dst);
    assert_eq!(dst, "Hello, world!");
}

#[test]
fn test_no_expansion() {
    struct TestNoExpansion<'t>(&'t str);

    impl<'t> Replacer for TestNoExpansion<'t> {
        fn replace_append(&mut self, _: &Captures, dst: &mut String) {
            dst.push_str(self.0);
        }

        fn no_expansion<'r>(&'r mut self) -> Option<Cow<str>> {
            Some(Cow::Borrowed("No expansion"))
        }
    }

    let text_input = "text";
    let mut dst = String::new();
    let mut replacer = TestNoExpansion(text_input);

    let locs = Locations::default(); // Assuming a default constructor exists
    let named_groups = Arc::new(HashMap::new()); // Using an empty HashMap for simplicity
    let captures = Captures {
        text: "sample",
        locs,
        named_groups,
    };

    replacer.replace_append(&captures, &mut dst);
    assert_eq!(dst, "text");
    assert_eq!(replacer.no_expansion().unwrap(), Cow::Borrowed("No expansion"));
}

