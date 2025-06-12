// Answer 0

#[derive(Default)]
struct TestStruct {
    nonce: u64,
}

impl TestStruct {
    pub fn get_stream_param(&self, param: u64) -> u64 {
        // Assume STREAM_PARAM_NONCE is defined as a constant somewhere
        const STREAM_PARAM_NONCE: u64 = 42; // Placeholder value, modify as needed
        if param == STREAM_PARAM_NONCE {
            self.nonce
        } else {
            0
        }
    }

    pub fn get_nonce(&self) -> u64 {
        self.get_stream_param(STREAM_PARAM_NONCE)
    }
}

#[test]
fn test_get_nonce_correct_value() {
    let test_instance = TestStruct { nonce: 42 }; // Set nonce to a known value
    assert_eq!(test_instance.get_nonce(), 42);
}

#[test]
fn test_get_nonce_default_value() {
    let test_instance = TestStruct::default(); // Create instance with default values
    assert_eq!(test_instance.get_nonce(), 0); // No nonce set
}

