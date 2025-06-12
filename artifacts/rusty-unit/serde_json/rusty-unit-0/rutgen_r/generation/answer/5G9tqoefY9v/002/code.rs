// Answer 0

#[test]
fn test_skip_to_escape_empty_string() {
    let mut index = 0;
    let slice = b"";
    let forbid_control_characters = true;

    let mut instance = TestStruct { index, slice };
    instance.skip_to_escape(forbid_control_characters);

    assert_eq!(instance.index, 0);
}

#[test]
fn test_skip_to_escape_consecutive_escapes() {
    let mut index = 0;
    let slice = b"\\u041b\\u0435";
    let forbid_control_characters = false;

    let mut instance = TestStruct { index, slice };
    instance.skip_to_escape(forbid_control_characters);

    assert_eq!(instance.index, 0);
}

#[test]
fn test_skip_to_escape_forbidden_control_character() {
    let mut index = 0;
    let slice = b"abc\x1Fdef";
    let forbid_control_characters = true;

    let mut instance = TestStruct { index, slice };
    instance.skip_to_escape(forbid_control_characters);

    assert_eq!(instance.index, 3);
}

#[test]
fn test_skip_to_escape_valid_string() {
    let mut index = 0;
    let slice = b"abc\"def\\gh";
    let forbid_control_characters = false;

    let mut instance = TestStruct { index, slice };
    instance.skip_to_escape(forbid_control_characters);

    assert_eq!(instance.index, 7);
}

#[test]
fn test_skip_to_escape_with_index_out_of_bounds() {
    let mut index = 5;
    let slice = b"abc";
    let forbid_control_characters = false;

    let mut instance = TestStruct { index, slice };
    instance.skip_to_escape(forbid_control_characters);

    assert_eq!(instance.index, 5);
}

struct TestStruct<'a> {
    index: usize,
    slice: &'a [u8],
}

fn is_escape(byte: u8, forbid_control_characters: bool) -> bool {
    if forbid_control_characters {
        byte == b'\\' || (0x00..=0x1F).contains(&byte)
    } else {
        byte == b'\\'
    }
}

