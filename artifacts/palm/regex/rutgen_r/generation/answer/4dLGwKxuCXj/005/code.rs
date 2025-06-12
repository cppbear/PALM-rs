// Answer 0

#[test]
fn test_pos_valid_capture_group() {
    struct CaptureGroup(Vec<Option<usize>>);
    
    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let cg = CaptureGroup(vec![Some(0), Some(5), Some(10), Some(15)]);
    
    assert_eq!(cg.pos(1), Some((5, 10)));
}

#[test]
fn test_pos_invalid_capture_group() {
    struct CaptureGroup(Vec<Option<usize>>);
    
    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let cg = CaptureGroup(vec![None, Some(5), Some(10), Some(15)]);
    
    assert_eq!(cg.pos(0), None);
}

#[test]
fn test_pos_out_of_bounds() {
    struct CaptureGroup(Vec<Option<usize>>);
    
    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let cg = CaptureGroup(vec![Some(0), Some(5)]); // Only 2 capture groups
    
    assert_eq!(cg.pos(2), None);
}

#[test]
fn test_pos_no_matches() {
    struct CaptureGroup(Vec<Option<usize>>);
    
    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let cg = CaptureGroup(vec![Some(0), Some(5), Some(10), None]); // Third group has no match
    
    assert_eq!(cg.pos(3), None);
}

