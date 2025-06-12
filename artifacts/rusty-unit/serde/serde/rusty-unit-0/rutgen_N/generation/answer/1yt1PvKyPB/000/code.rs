// Answer 0

#[derive(Debug)]
struct SeqAccessDeserializer<A> {
    seq: A,
}

impl<A> SeqAccessDeserializer<A> {
    pub fn new(seq: A) -> Self {
        SeqAccessDeserializer { seq }
    }
}

#[test]
fn test_seq_access_deserializer_new() {
    let my_seq = vec![1, 2, 3];
    let deserializer = SeqAccessDeserializer::new(my_seq);
    
    assert_eq!(deserializer.seq, vec![1, 2, 3]);
}

#[test]
fn test_seq_access_deserializer_with_empty_sequence() {
    let my_seq: Vec<i32> = Vec::new();
    let deserializer = SeqAccessDeserializer::new(my_seq);
    
    assert_eq!(deserializer.seq, Vec::<i32>::new());
}

