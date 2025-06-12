fn new(num_byte_classes: usize) -> Transitions {
        Transitions {
            table: vec![],
            num_byte_classes: num_byte_classes,
        }
    }