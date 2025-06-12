// Answer 0

#[derive(Debug)]
struct Deserializer<R> {
    reader: R,
}

struct SeqAccess<'a, R> {
    de: &'a mut Deserializer<R>,
    first: bool,
}

impl<R> SeqAccess<'_, R> {
    fn new(de: &mut Deserializer<R>) -> Self {
        SeqAccess { de, first: true }
    }
}

#[test]
fn test_seq_access_new() {
    let reader = vec![1, 2, 3]; // Example data to simulate R
    let mut deserializer = Deserializer { reader };
    let seq_access = SeqAccess::new(&mut deserializer);
    assert_eq!(seq_access.first, true);
}

