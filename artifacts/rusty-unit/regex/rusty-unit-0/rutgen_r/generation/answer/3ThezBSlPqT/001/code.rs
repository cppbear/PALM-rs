// Answer 0

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct NFA {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct Regex {
        ro: RO,
    }

    struct RO {
        nfa: NFA,
    }

    impl Regex {
        pub fn new(groups: HashMap<String, usize>) -> Self {
            let nfa = NFA {
                capture_name_idx: Arc::new(groups),
            };
            let ro = RO { nfa };
            Regex { ro }
        }

        pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
            &self.ro.nfa.capture_name_idx
        }
    }

    #[test]
    fn test_capture_name_idx_non_empty() {
        let mut groups = HashMap::new();
        groups.insert("group1".to_string(), 0);
        groups.insert("group2".to_string(), 1);
        let regex = Regex::new(groups);
        
        let capture_idx = regex.capture_name_idx();
        
        assert_eq!(capture_idx.get("group1"), Some(&0));
        assert_eq!(capture_idx.get("group2"), Some(&1));
    }

    #[test]
    fn test_capture_name_idx_empty() {
        let groups = HashMap::new();
        let regex = Regex::new(groups);
        
        let capture_idx = regex.capture_name_idx();
        
        assert!(capture_idx.is_empty());
    }

    #[test]
    fn test_capture_name_idx_single_element() {
        let mut groups = HashMap::new();
        groups.insert("group1".to_string(), 0);
        let regex = Regex::new(groups);

        let capture_idx = regex.capture_name_idx();
        
        assert_eq!(capture_idx.get("group1"), Some(&0));
        assert_eq!(capture_idx.len(), 1);
    }

    #[test]
    fn test_capture_name_idx_multiple_elements() {
        let mut groups = HashMap::new();
        groups.insert("group1".to_string(), 0);
        groups.insert("group2".to_string(), 1);
        groups.insert("group3".to_string(), 2);
        let regex = Regex::new(groups);

        let capture_idx = regex.capture_name_idx();
        
        assert_eq!(capture_idx.len(), 3);
        assert_eq!(capture_idx.get("group3"), Some(&2));
    }
}

