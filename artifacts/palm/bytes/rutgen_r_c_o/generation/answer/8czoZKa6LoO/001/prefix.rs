// Answer 0

#[test]
fn test_into_inner_empty_buffer() {
    let buf: Vec<u8> = vec![];
    let writer = Writer { buf };
    let inner_buf = writer.into_inner();
}

#[test]
fn test_into_inner_small_buffer() {
    let buf: Vec<u8> = vec![1, 2, 3];
    let writer = Writer { buf };
    let inner_buf = writer.into_inner();
}

#[test]
fn test_into_inner_medium_buffer() {
    let buf: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let writer = Writer { buf };
    let inner_buf = writer.into_inner();
}

#[test]
fn test_into_inner_large_buffer() {
    let buf: Vec<u8> = (0..512).map(|i| i as u8).collect();
    let writer = Writer { buf };
    let inner_buf = writer.into_inner();
}

#[test]
#[should_panic]
fn test_into_inner_overflow() {
    let buf: Vec<u8> = (0..513).map(|i| i as u8).collect();
    let writer = Writer { buf };
    let inner_buf = writer.into_inner();
}

