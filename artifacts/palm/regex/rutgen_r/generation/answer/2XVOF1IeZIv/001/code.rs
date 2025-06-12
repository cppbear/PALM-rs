// Answer 0

fn test_fmt_cut() {
    struct Literal {
        v: String,
        cut: bool,
    }

    impl Literal {
        fn is_cut(&self) -> bool {
            self.cut
        }
        
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.is_cut() {
                write!(f, "Cut({})", escape_unicode(&self.v))
            } else {
                write!(f, "Complete({})", escape_unicode(&self.v))
            }
        }
    }

    fn escape_unicode(s: &str) -> String {
        s.chars().flat_map(|c| format!("{:x}", c as u32)).collect::<Vec<_>>().join("\\u{") + "}"
    }

    let lit = Literal {
        v: String::from("Test"),
        cut: true,
    };

    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        lit.fmt(formatter).unwrap();
    }

    assert_eq!(output, "Cut(Test)");
}

fn test_fmt_complete() {
    struct Literal {
        v: String,
        cut: bool,
    }

    impl Literal {
        fn is_cut(&self) -> bool {
            self.cut
        }

        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.is_cut() {
                write!(f, "Cut({})", escape_unicode(&self.v))
            } else {
                write!(f, "Complete({})", escape_unicode(&self.v))
            }
        }
    }

    fn escape_unicode(s: &str) -> String {
        s.chars().flat_map(|c| format!("{:x}", c as u32)).collect::<Vec<_>>().join("\\u{") + "}"
    }

    let lit = Literal {
        v: String::from("Test"),
        cut: false,
    };

    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        lit.fmt(formatter).unwrap();
    }

    assert_eq!(output, "Complete(Test)");
}

fn test_fmt_unicode() {
    struct Literal {
        v: String,
        cut: bool,
    }

    impl Literal {
        fn is_cut(&self) -> bool {
            self.cut
        }

        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.is_cut() {
                write!(f, "Cut({})", escape_unicode(&self.v))
            } else {
                write!(f, "Complete({})", escape_unicode(&self.v))
            }
        }
    }

    fn escape_unicode(s: &str) -> String {
        s.chars().flat_map(|c| format!("{:x}", c as u32)).collect::<Vec<_>>().join("\\u{") + "}"
    }

    let lit = Literal {
        v: String::from("测试"), // Chinese characters
        cut: true,
    };

    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        lit.fmt(formatter).unwrap();
    }

    assert_eq!(output, "Cut(\\u{6d4b}\\u{8bd5})"); // Expecting unicode representation
}

