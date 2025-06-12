// Answer 0

#[test]
fn test_replace_all_with_static_string_replacer() {
    struct StaticReplacer<'a>(&'a [u8]);
    
    impl<'a> Replacer for StaticReplacer<'a> {
        fn no_expansion(&self) -> Option<&'a [u8]> {
            Some(self.0)
        }
        
        fn replace_append(&self, _captures: &Captures, output: &mut Vec<u8>) {
            output.extend_from_slice(self.0);
        }
    }

    let regex = Regex::new(r"foo").unwrap();
    let text = b"foo bar foo baz";
    let replacer = StaticReplacer(b"bar");

    let result = regex.replace_all(text, replacer);
    assert_eq!(result, Cow::Owned(b"bar bar bar baz".to_vec()));
}

#[test]
fn test_replace_all_with_empty_replacer() {
    struct EmptyReplacer;

    impl Replacer for EmptyReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(b"")
        }
        
        fn replace_append(&self, _captures: &Captures, output: &mut Vec<u8>) {
            // No replacement performed
        }
    }

    let regex = Regex::new(r"test").unwrap();
    let text = b"this is a test string";
    let replacer = EmptyReplacer;

    let result = regex.replace_all(text, replacer);
    assert_eq!(result, Cow::Owned(b"this is a  string".to_vec()));
}

#[test]
fn test_replace_all_with_no_match() {
    struct StaticReplacer<'a>(&'a [u8]);

    impl<'a> Replacer for StaticReplacer<'a> {
        fn no_expansion(&self) -> Option<&'a [u8]> {
            Some(self.0)
        }

        fn replace_append(&self, _captures: &Captures, output: &mut Vec<u8>) {
            output.extend_from_slice(self.0);
        }
    }

    let regex = Regex::new(r"abc").unwrap();
    let text = b"no match here";
    let replacer = StaticReplacer(b"def");

    let result = regex.replace_all(text, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

