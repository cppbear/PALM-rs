// Answer 0

#[test]
fn test_find_at_valid_match() {
    struct SimpleExec;
    
    impl SimpleExec {
        fn searcher_str(&self) -> ExecNoSyncStr {
            // Mock implementation goes here
            ExecNoSyncStr(self)
        }
        
        fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<(usize, usize)> {
            if start > text.len() {
                return None;
            }
            let match_start = text[start..].find('a').map(|pos| pos + start);
            match_start.map(|s| (s, s + 1))
        }
    }
    
    let regex = Regex(SimpleExec);
    let result = regex.find_at("banana", 1);
    assert_eq!(result, Some(Match { text: "banana", start: 1, end: 2 }));
}

#[test]
fn test_find_at_no_match() {
    struct SimpleExec;
    
    impl SimpleExec {
        fn searcher_str(&self) -> ExecNoSyncStr {
            ExecNoSyncStr(self)
        }
        
        fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<(usize, usize)> {
            if start > text.len() {
                return None;
            }
            let match_start = text[start..].find('z').map(|pos| pos + start);
            match_start.map(|s| (s, s + 1))
        }
    }
    
    let regex = Regex(SimpleExec);
    let result = regex.find_at("banana", 0);
    assert_eq!(result, None);
}

#[test]
fn test_find_at_start_out_of_bounds() {
    struct SimpleExec;
    
    impl SimpleExec {
        fn searcher_str(&self) -> ExecNoSyncStr {
            ExecNoSyncStr(self)
        }
        
        fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<(usize, usize)> {
            if start > text.len() {
                return None;
            }
            if let Some(pos) = text[start..].find('a') {
                return Some((start + pos, start + pos + 1));
            }
            None
        }
    }
    
    let regex = Regex(SimpleExec);
    let result = regex.find_at("banana", 7);
    assert_eq!(result, None);
}

#[test]
fn test_find_at_empty_string() {
    struct SimpleExec;
    
    impl SimpleExec {
        fn searcher_str(&self) -> ExecNoSyncStr {
            ExecNoSyncStr(self)
        }
        
        fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<(usize, usize)> {
            if start > text.len() {
                return None;
            }
            if let Some(pos) = text[start..].find('a') {
                return Some((start + pos, start + pos + 1));
            }
            None
        }
    }
    
    let regex = Regex(SimpleExec);
    let result = regex.find_at("", 0);
    assert_eq!(result, None);
}

