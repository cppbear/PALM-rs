// Answer 0

#[test]
fn test_captures_iter() {
    struct MockExec {
        // Mock necessary fields here if required for testing
    }
    
    impl Exec {
        fn searcher_str(&self) -> ExecNoSyncStr {
            // Return a mock ExecNoSyncStr for testing
            ExecNoSyncStr(MockExec {})
        }
    }
    
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    
    let mut captures_iter = re.captures_iter(text);
    
    let first_capture = captures_iter.next().unwrap();
    assert_eq!(&first_capture["title"], "Citizen Kane");
    assert_eq!(&first_capture["year"], "1941");
    
    let second_capture = captures_iter.next().unwrap();
    assert_eq!(&second_capture["title"], "The Wizard of Oz");
    assert_eq!(&second_capture["year"], "1939");
    
    let third_capture = captures_iter.next().unwrap();
    assert_eq!(&third_capture["title"], "M");
    assert_eq!(&third_capture["year"], "1931");
    
    assert!(captures_iter.next().is_none());
}

