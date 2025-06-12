// Answer 0

#[test]
fn test_find_at_start_zero() {
    struct SimpleRegex(String);
    
    impl SimpleRegex {
        fn new(pattern: &str) -> Self {
            SimpleRegex(pattern.to_string())
        }
        
        fn searcher_str(&self) -> &str {
            &self.0
        }
    }
    
    impl SimpleRegex {
        pub fn find_at<'t>(
            &self,
            text: &'t str,
            start: usize,
        ) -> Option<(&'t str, &'t str)> {
            if start == 0 && text.contains(&self.0) {
                let start_idx = text.find(&self.0).unwrap();
                let end_idx = start_idx + self.0.len();
                Some((&text[start_idx..end_idx], ""))
            } else {
                None
            }
        }
    }
    
    let re = SimpleRegex::new("test");
    let text = "test string";
    
    let result = re.find_at(text, 0);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, "test");
}

#[test]
fn test_find_at_start_non_zero() {
    struct SimpleRegex(String);
    
    impl SimpleRegex {
        fn new(pattern: &str) -> Self {
            SimpleRegex(pattern.to_string())
        }
        
        fn searcher_str(&self) -> &str {
            &self.0
        }
    }
    
    impl SimpleRegex {
        pub fn find_at<'t>(
            &self,
            text: &'t str,
            start: usize,
        ) -> Option<(&'t str, &'t str)> {
            if start == 0 && text.contains(&self.0) {
                let start_idx = text.find(&self.0).unwrap();
                let end_idx = start_idx + self.0.len();
                Some((&text[start_idx..end_idx], ""))
            } else {
                None
            }
        }
    }
    
    let re = SimpleRegex::new("test");
    let text = "test string";
    
    let result = re.find_at(text, 1);
    
    assert!(result.is_none());
}

#[test]
fn test_find_at_pattern_not_found() {
    struct SimpleRegex(String);
    
    impl SimpleRegex {
        fn new(pattern: &str) -> Self {
            SimpleRegex(pattern.to_string())
        }
        
        fn searcher_str(&self) -> &str {
            &self.0
        }
    }
    
    impl SimpleRegex {
        pub fn find_at<'t>(
            &self,
            text: &'t str,
            start: usize,
        ) -> Option<(&'t str, &'t str)> {
            if start == 0 && text.contains(&self.0) {
                let start_idx = text.find(&self.0).unwrap();
                let end_idx = start_idx + self.0.len();
                Some((&text[start_idx..end_idx], ""))
            } else {
                None
            }
        }
    }
    
    let re = SimpleRegex::new("notfound");
    let text = "test string";
    
    let result = re.find_at(text, 0);
    
    assert!(result.is_none());
}

