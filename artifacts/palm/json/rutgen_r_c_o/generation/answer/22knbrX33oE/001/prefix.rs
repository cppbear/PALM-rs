// Answer 0

#[test]
fn test_byte_offset_zero() {
    let data: &[u8] = b"[0]";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
}

#[test]
fn test_byte_offset_multiple_elements() {
    let data: &[u8] = b"[0, 1, 2]";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
}

#[test]
fn test_byte_offset_end_of_file() {
    let data: &[u8] = b"[0, 1, 2, ";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
}

#[test]
fn test_byte_offset_large_input() {
    let data: &[u8] = b"[0]".repeat(1000).as_bytes(); // large input
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    for _ in 0..1000 {
        println!("{:?}", stream.next());
    }
    println!("{}", stream.byte_offset());
}

#[test]
fn test_byte_offset_empty_data() {
    let data: &[u8] = b""; // empty input
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
}

#[test]
fn test_byte_offset_edge_value() {
    let data: &[u8] = b"[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]"; // multiple elements
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    for _ in 0..10 {
        println!("{:?}", stream.next());
        println!("{}", stream.byte_offset());
    }
}

#[test]
fn test_byte_offset_large_numbers() {
    let data: &[u8] = b"[1000000000, 2000000000, 3000000000]"; // large numbers
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
    println!("{:?}", stream.next());
    println!("{}", stream.byte_offset());
}

