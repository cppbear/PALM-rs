// Answer 0

#[derive(Debug)]
struct MyRegex<R> {
    regex: R,
}

impl<R> MyRegex<R> {
    pub fn new(regex: R) -> Self {
        MyRegex { regex }
    }

    pub fn regex(&self) -> &R {
        &self.regex
    }
}

#[derive(Debug)]
struct Wrapper<R>(MyRegex<R>);

impl<R> Wrapper<R> {
    pub fn new(regex: R) -> Self {
        Wrapper(MyRegex::new(regex))
    }

    pub fn regex(&self) -> &R {
        self.0.regex()
    }
}

#[test]
fn test_regex() {
    let regex = Wrapper::new("abc");
    
    assert_eq!(regex.regex(), &"abc");
}

#[test]
fn test_regex_empty() {
    let regex = Wrapper::new("");
    
    assert_eq!(regex.regex(), &"");
}

