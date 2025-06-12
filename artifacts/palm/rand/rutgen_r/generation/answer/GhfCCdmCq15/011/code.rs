// Answer 0

#[test]
fn test_append_string_max_char_length() {
    use rand::rngs::ThreadRng;
    use rand::Rng;
    
    struct SampleStruct {
        slice: Vec<&'static str>,
    }

    impl SampleStruct {
        fn new(slice: Vec<&'static str>) -> Self {
            SampleStruct { slice }
        }

        fn sample_iter(&self, rng: &mut ThreadRng) -> impl Iterator<Item = char> {
            self.slice.iter().flat_map(|s| s.chars()).cycle()
        }
    }

    let mut rng = rand::thread_rng();
    let sample = SampleStruct::new(vec!["a"]);
    let mut result_string = String::new();
    
    // Input constraints
    sample.append_string(&mut rng, &mut result_string, 100);
    
    // Check the result length
    assert_eq!(result_string.len(), 100);
}

#[test]
#[should_panic]
fn test_append_string_zero_extend_len() {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    struct SampleStruct {
        slice: Vec<&'static str>,
    }

    impl SampleStruct {
        fn new(slice: Vec<&'static str>) -> Self {
            SampleStruct { slice }
        }

        fn sample_iter(&self, rng: &mut ThreadRng) -> impl Iterator<Item = char> {
            self.slice.iter().flat_map(|s| s.chars()).cycle()
        }
    }

    let mut rng = rand::thread_rng();
    let sample = SampleStruct::new(vec!["abc"]);
    let mut result_string = String::new();

    // Set conditions to potentially cause panic: edge case of having no extendable length
    sample.append_string(&mut rng, &mut result_string, 10); // len is valid but extend_len will be 0
}

