// Answer 0

#[test]
fn test_from_bytes_length_4_not_post() {
    let input1 = b"HEAD";
    let _ = Method::from_bytes(input1);
    
    let input2 = b"TEST";
    let _ = Method::from_bytes(input2);
    
    let input3 = b"XXXX";
    let _ = Method::from_bytes(input3);
    
    let input4 = b"ABCD";
    let _ = Method::from_bytes(input4);
}

