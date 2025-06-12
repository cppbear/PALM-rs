// Answer 0

#[derive(Debug)]
struct U32Deserializer {
    value: u32,
    marker: std::marker::PhantomData<()>,
}

impl U32Deserializer {
    pub fn new(value: u32) -> Self {
        U32Deserializer {
            value,
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_new_u32_deserializer() {
    let value: u32 = 42;
    let deserializer = U32Deserializer::new(value);
    
    assert_eq!(deserializer.value, value);
}

#[test]
fn test_new_u32_deserializer_boundary() {
    let min_value: u32 = 0;
    let max_value: u32 = u32::MAX;
    
    let deserializer_min = U32Deserializer::new(min_value);
    assert_eq!(deserializer_min.value, min_value);
    
    let deserializer_max = U32Deserializer::new(max_value);
    assert_eq!(deserializer_max.value, max_value);
}

