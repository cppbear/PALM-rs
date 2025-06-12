// Answer 0

#[test]
fn test_find_two_dense_chars_in_text() {
    let mut single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![65, 66], // Characters 'A' and 'B'
        complete: false,
        all_ascii: true,
    };
    single_byte_set.sparse[65] = true; // Mark 'A' as present
    single_byte_set.sparse[66] = true; // Mark 'B' as present
    let text = b"Hello, AB world!";
    let result = single_byte_set.find(text);
}

#[test]
fn test_find_two_dense_chars_at_start_of_text() {
    let mut single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![67, 68], // Characters 'C' and 'D'
        complete: false,
        all_ascii: true,
    };
    single_byte_set.sparse[67] = true; // Mark 'C' as present
    single_byte_set.sparse[68] = true; // Mark 'D' as present
    let text = b"CD is the start.";
    let result = single_byte_set.find(text);
}

#[test]
fn test_find_two_dense_chars_with_repeated_characters() {
    let mut single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![69, 70], // Characters 'E' and 'F'
        complete: false,
        all_ascii: true,
    };
    single_byte_set.sparse[69] = true; // Mark 'E' as present
    single_byte_set.sparse[70] = true; // Mark 'F' as present
    let text = b"EEEF is better than F. FEE.";
    let result = single_byte_set.find(text);
}

#[test]
fn test_find_two_dense_chars_not_in_text() {
    let mut single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![71, 72], // Characters 'G' and 'H'
        complete: false,
        all_ascii: true,
    };
    single_byte_set.sparse[71] = true; // Mark 'G' as present
    single_byte_set.sparse[72] = true; // Mark 'H' as present
    let text = b"Nothing here.";
    let result = single_byte_set.find(text);
}

#[test]
fn test_find_two_dense_chars_at_text_end() {
    let mut single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![73, 74], // Characters 'I' and 'J'
        complete: false,
        all_ascii: true,
    };
    single_byte_set.sparse[73] = true; // Mark 'I' as present
    single_byte_set.sparse[74] = true; // Mark 'J' as present
    let text = b"Hello, this is a test IJ";
    let result = single_byte_set.find(text);
}

#[test]
fn test_find_empty_text() {
    let mut single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![75, 76], // Characters 'K' and 'L'
        complete: false,
        all_ascii: true,
    };
    single_byte_set.sparse[75] = true; // Mark 'K' as present
    single_byte_set.sparse[76] = true; // Mark 'L' as present
    let text = b"";
    let result = single_byte_set.find(text);
}

