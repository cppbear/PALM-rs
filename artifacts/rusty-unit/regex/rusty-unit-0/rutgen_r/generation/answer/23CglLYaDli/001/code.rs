// Answer 0

#[test]
fn test_u8_class_valid_lower_bound() {
    struct Prog {
        byte_classes: [u8; 256],
    }

    struct Regex {
        prog: Prog,
    }

    impl Regex {
        fn u8_class(&self, b: u8) -> usize {
            self.prog.byte_classes[b as usize] as usize
        }
    }

    let prog = Prog { byte_classes: [1; 256] };
    let regex = Regex { prog };

    assert_eq!(regex.u8_class(0), 1);
}

#[test]
fn test_u8_class_valid_upper_bound() {
    struct Prog {
        byte_classes: [u8; 256],
    }

    struct Regex {
        prog: Prog,
    }

    impl Regex {
        fn u8_class(&self, b: u8) -> usize {
            self.prog.byte_classes[b as usize] as usize
        }
    }

    let prog = Prog { byte_classes: [1; 256] };
    let regex = Regex { prog };

    assert_eq!(regex.u8_class(255), 1);
}

#[test]
#[should_panic]
fn test_u8_class_out_of_bounds_negative() {
    struct Prog {
        byte_classes: [u8; 256],
    }

    struct Regex {
        prog: Prog,
    }

    impl Regex {
        fn u8_class(&self, b: u8) -> usize {
            self.prog.byte_classes[b as usize] as usize
        }
    }

    let prog = Prog { byte_classes: [1; 256] };
    let regex = Regex { prog };

    // This will not panic but is here to test the implementation beyond typical u8
    regex.u8_class(256); // Invalid input, just exceeding the range of u8
}

#[test]
#[should_panic]
fn test_u8_class_out_of_bounds_high() {
    struct Prog {
        byte_classes: [u8; 256],
    }

    struct Regex {
        prog: Prog,
    }

    impl Regex {
        fn u8_class(&self, b: u8) -> usize {
            self.prog.byte_classes[b as usize] as usize
        }
    }

    let prog = Prog { byte_classes: [1; 256] };
    let regex = Regex { prog };

    // Invalid input scenario exceeding u8 limits (only for testing)
    regex.u8_class(255 + 1); // Input exceeds valid range of u8
}

