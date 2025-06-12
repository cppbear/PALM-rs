// Answer 0

#[test]
fn test_pos_valid_capture_group() {
    struct CaptureGroup(Option<[Option<usize>; 4]>);

    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let capture_group = CaptureGroup(Some([Some(0), Some(5), None, Some(10)]));
    let result = capture_group.pos(1);
    assert_eq!(result, Some((5, 10)));
}

#[test]
fn test_pos_invalid_capture_group_index() {
    struct CaptureGroup(Option<[Option<usize>; 4]>);

    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let capture_group = CaptureGroup(Some([Some(0), Some(5), None, Some(10)]));
    let result = capture_group.pos(2);
    assert_eq!(result, None);
}

#[test]
fn test_pos_no_capture() {
    struct CaptureGroup(Option<[Option<usize>; 4]>);

    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let capture_group = CaptureGroup(Some([None, None, None, None]));
    let result = capture_group.pos(1);
    assert_eq!(result, None);
}

#[test]
fn test_pos_out_of_bounds_index() {
    struct CaptureGroup(Option<[Option<usize>; 4]>);

    impl CaptureGroup {
        pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
            let (s, e) = (i * 2, i * 2 + 1);
            match (self.0.get(s), self.0.get(e)) {
                (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
                _ => None,
            }
        }
    }

    let capture_group = CaptureGroup(Some([Some(0), Some(5), Some(10), Some(15)]));
    let result = capture_group.pos(3);
    assert_eq!(result, None);
}

