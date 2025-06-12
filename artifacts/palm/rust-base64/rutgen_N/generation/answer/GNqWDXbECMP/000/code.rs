// Answer 0

#[derive(Debug)]
struct Encoder {
    extra_input: String,
    extra_input_occupied_len: usize,
    output: Vec<u8>,
    output_occupied_len: usize,
}

impl Encoder {
    fn new(extra_input: String, output: Vec<u8>) -> Self {
        let extra_input_occupied_len = extra_input.len();
        let output_occupied_len = output.len();
        Self {
            extra_input,
            extra_input_occupied_len,
            output,
            output_occupied_len,
        }
    }
}

impl std::fmt::Display for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "extra_input: {:?} extra_input_occupied_len:{:?} output[..5]: {:?} output_occupied_len: {:?}",
            self.extra_input,
            self.extra_input_occupied_len,
            &self.output[0..5],
            self.output_occupied_len
        )
    }
}

#[test]
fn test_encoder_display_with_empty_extra_input() {
    let encoder = Encoder::new("".to_string(), vec![1, 2, 3, 4, 5, 6]);
    let result = format!("{}", encoder);
    assert_eq!(result, "extra_input: \"\" extra_input_occupied_len:0 output[..5]: [1, 2, 3, 4, 5] output_occupied_len: 6");
}

#[test]
fn test_encoder_display_with_non_empty_extra_input() {
    let encoder = Encoder::new("Hello".to_string(), vec![1, 2, 3, 4, 5, 6]);
    let result = format!("{}", encoder);
    assert_eq!(result, "extra_input: \"Hello\" extra_input_occupied_len:5 output[..5]: [1, 2, 3, 4, 5] output_occupied_len: 6");
}

#[test]
fn test_encoder_display_with_large_output() {
    let encoder = Encoder::new("World".to_string(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let result = format!("{}", encoder);
    assert_eq!(result, "extra_input: \"World\" extra_input_occupied_len:5 output[..5]: [0, 1, 2, 3, 4] output_occupied_len: 10");
}

#[test]
fn test_encoder_display_with_five_byte_output() {
    let encoder = Encoder::new("Test".to_string(), vec![10, 20, 30, 40, 50]);
    let result = format!("{}", encoder);
    assert_eq!(result, "extra_input: \"Test\" extra_input_occupied_len:4 output[..5]: [10, 20, 30, 40, 50] output_occupied_len: 5");
}

