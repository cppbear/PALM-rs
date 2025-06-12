// Answer 0

#[derive(Default)]
struct NfaEngine {
    bytes: bool,
}

impl NfaEngine {
    pub fn bytes(mut self, yes: bool) -> Self {
        self.bytes = yes;
        self
    }
}

#[test]
fn test_bytes_set_to_true() {
    let engine = NfaEngine::default().bytes(true);
    assert!(engine.bytes);
}

#[test]
fn test_bytes_set_to_false() {
    let engine = NfaEngine::default().bytes(false);
    assert!(!engine.bytes);
}

#[test]
fn test_bytes_initial_value() {
    let engine = NfaEngine::default();
    assert!(!engine.bytes);
}

