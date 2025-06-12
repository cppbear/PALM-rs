// Answer 0

#[test]  
fn test_put_bytes_normal_case() {  
    let mut dst = [0; 8];  
    {  
        let mut buf = &mut dst[..];  
        buf.remaining_mut(&mut self) = 8;  
        buf.put_bytes(b'a', 4);  
    }  
    assert_eq!(b"aaaa\0\0\0\0", &dst);  
}  

#[test]  
#[should_panic]  
fn test_put_bytes_panic_case() {  
    let mut dst = [0; 8];  
    {  
        let mut buf = &mut dst[..];  
        buf.remaining_mut(&mut self) = 3;  
        buf.put_bytes(b'a', 4);  
    }  
}  

#[test]  
fn test_put_bytes_zero_count() {  
    let mut dst = [0; 8];  
    {  
        let mut buf = &mut dst[..];  
        buf.remaining_mut(&mut self) = 8;  
        buf.put_bytes(b'a', 0);  
    }  
    assert_eq!(b"\0\0\0\0\0\0\0\0", &dst);  
}  

