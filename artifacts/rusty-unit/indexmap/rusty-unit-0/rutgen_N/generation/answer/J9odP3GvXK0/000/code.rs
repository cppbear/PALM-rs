// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Splice<'a> {
        drain: &'a str,
        replace_with: &'a str,
    }

    impl fmt::Debug for Splice<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Splice")
                .field("drain", &self.drain)
                .field("replace_with", &self.replace_with)
                .finish()
        }
    }

    let splice = Splice {
        drain: "drain_data",
        replace_with: "replacement_data",
    };

    let result = format!("{:?}", splice);
    assert_eq!(result, "Splice { drain: \"drain_data\", replace_with: \"replacement_data\" }");
}

#[test]
fn test_fmt_empty() {
    use std::fmt;

    struct Splice<'a> {
        drain: &'a str,
        replace_with: &'a str,
    }

    impl fmt::Debug for Splice<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Splice")
                .field("drain", &self.drain)
                .field("replace_with", &self.replace_with)
                .finish()
        }
    }

    let splice = Splice {
        drain: "",
        replace_with: "",
    };

    let result = format!("{:?}", splice);
    assert_eq!(result, "Splice { drain: \"\", replace_with: \"\" }");
}

