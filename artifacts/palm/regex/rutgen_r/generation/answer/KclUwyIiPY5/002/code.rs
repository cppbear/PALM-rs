// Answer 0

#[test]
fn test_is_end_with_c_not_none_and_byte_none() {
    struct Input {
        c: Option<char>,
        byte: Option<u8>,
    }

    let input = Input {
        c: Some('a'), // self.c.is_none() is false
        byte: None,   // self.byte is None
    };

    assert_eq!(input.is_end(), false); // expects false since c is not None
}

#[test]
fn test_is_end_with_c_not_none_and_byte_not_none() {
    struct Input {
        c: Option<char>,
        byte: Option<u8>,
    }

    let input = Input {
        c: Some('b'), // self.c.is_none() is false
        byte: Some(1), // self.byte is not None
    };

    assert_eq!(input.is_end(), false); // expects false since c is not None
}

#[test]
fn test_is_end_with_c_none_and_byte_not_none() {
    struct Input {
        c: Option<char>,
        byte: Option<u8>,
    }

    let input = Input {
        c: None,      // self.c.is_none() is true
        byte: Some(1), // self.byte is not None
    };

    assert_eq!(input.is_end(), false); // expects false because c is None

}

#[test]
fn test_is_end_with_c_none_and_byte_none() {
    struct Input {
        c: Option<char>,
        byte: Option<u8>,
    }

    let input = Input {
        c: None,      // self.c.is_none() is true
        byte: None,   // self.byte is None
    };

    assert_eq!(input.is_end(), true); // expects true since both are None
}

