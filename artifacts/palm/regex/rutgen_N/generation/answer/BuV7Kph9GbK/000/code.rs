// Answer 0

#[derive(Default)]
struct DepthTracker {
    depth: usize,
}

impl DepthTracker {
    fn decrement_depth(&mut self) {
        self.depth = self.depth.checked_sub(1).unwrap();
    }
}

#[test]
fn test_decrement_depth() {
    let mut tracker = DepthTracker::default();
    tracker.depth = 1; // Setting depth to 1 for testing
    tracker.decrement_depth();
    assert_eq!(tracker.depth, 0);
}

#[test]
#[should_panic]
fn test_decrement_depth_panics_below_zero() {
    let mut tracker = DepthTracker::default();
    tracker.depth = 0; // Setting depth to 0 to trigger panic
    tracker.decrement_depth();
}

