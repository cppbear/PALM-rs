// Answer 0

#[test]
fn test_num_byte_classes() {
    struct Prog {
        byte_classes: [u8; 256],
    }

    struct State {
        prog: Prog,
    }

    let prog = Prog {
        byte_classes: [0; 256], // Initializing all byte classes to 0
    };

    let state = State { prog };

    assert_eq!(state.num_byte_classes(), 1); // 0 byte classes + 1 for EOF + 1 for total
}

#[test]
fn test_num_byte_classes_max_byte_classes() {
    struct Prog {
        byte_classes: [u8; 256],
    }

    struct State {
        prog: Prog,
    }

    let mut byte_classes = [0; 256];
    byte_classes[255] = 5; // Setting the maximum number of byte classes

    let prog = Prog { byte_classes };

    let state = State { prog };

    assert_eq!(state.num_byte_classes(), 6); // 5 byte classes + 1 for EOF + 1 for total
}

