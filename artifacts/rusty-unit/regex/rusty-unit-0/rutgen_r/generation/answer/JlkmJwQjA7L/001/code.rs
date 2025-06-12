// Answer 0

#[derive(Debug)]
struct TestState {
    state_values: Vec<u32>,
}

impl TestState {
    fn new(state_values: Vec<u32>) -> Self {
        TestState { state_values }
    }
}

impl std::fmt::Debug for TestState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut fmtd = f.debug_map();
        for (b, si) in self.state_values.iter().enumerate() {
            match *si {
                0 => {}
                1 => {
                    fmtd.entry(&b, &"DEAD");
                }
                _ => {
                    fmtd.entry(&b, &si.to_string());
                }
            }
        }
        fmtd.finish()
    }
}

#[test]
fn test_debug_map_empty() {
    let state = TestState::new(vec![]);
    let result = format!("{:?}", state);
    assert_eq!(result, "{}");
}

#[test]
fn test_debug_map_with_dead_state() {
    let state = TestState::new(vec![1]);
    let result = format!("{:?}", state);
    assert_eq!(result, "{0: \"DEAD\"}");
}

#[test]
fn test_debug_map_with_unknown_and_dead_state() {
    let state = TestState::new(vec![0, 1]);
    let result = format!("{:?}", state);
    assert_eq!(result, "{1: \"DEAD\"}");
}

#[test]
fn test_debug_map_with_varied_states() {
    let state = TestState::new(vec![2, 0, 1, 3]);
    let result = format!("{:?}", state);
    assert_eq!(result, "{0: \"2\", 2: \"DEAD\", 3: \"3\"}");
}

#[test]
#[should_panic]
fn test_debug_map_with_large_vec() {
    let state = TestState::new(vec![0; 1000]); // All unknown states
    let result = format!("{:?}", state);
    assert_eq!(result, "{}"); // This shouldn't panic, but we test the boundary.
}

