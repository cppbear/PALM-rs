// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Regex {
        pattern: String,
    }

    impl fmt::Display for Regex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.pattern)
        }
    }

    let regex = Regex {
        pattern: String::from("a*b"),
    };

    let output = format!("{}", regex);
    assert_eq!(output, "a*b");
}

