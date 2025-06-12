// Answer 0

#[test]
fn test_chunk_non_empty() {
    struct ChunkStruct {
        data: Vec<u8>,
    }
    
    impl ChunkStruct {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    let buf = ChunkStruct {
        data: vec![1, 2, 3, 4, 5],
    };
    
    assert_eq!(buf.chunk(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_empty() {
    struct ChunkStruct {
        data: Vec<u8>,
    }
    
    impl ChunkStruct {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    let buf = ChunkStruct {
        data: vec![],
    };
    
    assert_eq!(buf.chunk(), &[]);
}

#[test]
fn test_chunk_single_element() {
    struct ChunkStruct {
        data: Vec<u8>,
    }
    
    impl ChunkStruct {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    let buf = ChunkStruct {
        data: vec![42],
    };
    
    assert_eq!(buf.chunk(), &[42]);
}

#[test]
#[should_panic]
fn test_chunk_panic() {
    struct ChunkStruct {}

    impl ChunkStruct {
        fn chunk(&self) -> &[u8] {
            panic!("This is a panic condition");
        }
    }

    let buf = ChunkStruct {};
    buf.chunk();
}

