// Answer 0

fn usize_to_u32(val: usize) -> u32 {
    val as u32
}

struct Memory {
    visited: Vec<u32>,
}

struct TestStruct {
    input: Vec<u8>,
    m: Memory,
}

impl TestStruct {
    fn new(input_size: usize, visited_size: usize) -> Self {
        Self {
            input: vec![0; input_size],
            m: Memory {
                visited: vec![0; visited_size],
            },
        }
    }

    fn has_visited(&mut self, ip: usize, at: &InputAt) -> bool {
        let k = ip * (self.input.len() + 1) + at.pos();
        let k1 = k / 32; // Assuming BIT_SIZE is 32
        let k2 = usize_to_u32(1 << (k % 32));
        if self.m.visited[k1] & k2 == 0 {
            self.m.visited[k1] |= k2;
            false
        } else {
            true
        }
    }
}

struct InputAt {
    position: usize,
}

impl InputAt {
    fn pos(&self) -> usize {
        self.position
    }
}

#[test]
fn test_has_visited_first_visit() {
    let mut test_struct = TestStruct::new(10, 5); // input_size = 10, visited_size = 5
    let at = InputAt { position: 0 };
    let result = test_struct.has_visited(0, &at);
    assert_eq!(result, false);
}

#[test]
fn test_has_visited_second_visit() {
    let mut test_struct = TestStruct::new(10, 5);
    let at = InputAt { position: 0 };
    
    // First visit
    let first_result = test_struct.has_visited(0, &at);
    assert_eq!(first_result, false);
    
    // Second visit
    let second_result = test_struct.has_visited(0, &at);
    assert_eq!(second_result, true);
}

#[test]
fn test_has_visited_boundary_condition() {
    let mut test_struct = TestStruct::new(10, 5);
    let at = InputAt { position: 4 }; // At the limit of the input size

    // Testing the edge case where position is at boundary
    let boundary_result = test_struct.has_visited(0, &at);
    assert_eq!(boundary_result, false);
    
    // Checking again to make sure it's now visited
    let repeat_result = test_struct.has_visited(0, &at);
    assert_eq!(repeat_result, true);
}

