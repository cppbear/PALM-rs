// Answer 0

#[derive(Default)]
struct MyStruct(Vec<u8>);

impl MyStruct {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        self.0.append(dst);
    }
}

#[derive(Default)]
struct Captures;

#[test]
fn test_replace_append_empty() {
    let mut my_struct = MyStruct::default();
    let caps = Captures::default();
    let mut dst = Vec::new();
    
    my_struct.replace_append(&caps, &mut dst);
    
    assert_eq!(my_struct.0.len(), 0);
    assert_eq!(dst.len(), 0);
}

#[test]
fn test_replace_append_with_data() {
    let mut my_struct = MyStruct(vec![1, 2, 3]);
    let caps = Captures::default();
    let mut dst = vec![4, 5, 6];
    
    my_struct.replace_append(&caps, &mut dst);
    
    assert_eq!(my_struct.0.len(), 3); // original length
    assert_eq!(dst, vec![4, 5, 6]); // dst remains unchanged
}

