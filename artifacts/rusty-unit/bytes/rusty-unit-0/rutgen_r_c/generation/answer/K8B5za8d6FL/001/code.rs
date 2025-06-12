// Answer 0

#[test]
fn test_chunk_ref_bounds() {
    struct TestCursor {
        position: u64,
        data: Vec<u8>,
    }

    impl std::io::Cursor<&[u8]> for TestCursor {
        fn remaining(&self) -> usize {
            (self.data.len() as u64).saturating_sub(self.position) as usize
        }

        fn chunk(&self) -> &[u8] {
            let slice = self.data.as_slice();
            let pos = min_u64_usize(self.position, slice.len());
            &slice[pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt as u64;
        }

        fn position(&self) -> u64 {
            self.position
        }
    }

    // Test case 1: Normal range
    let cursor = TestCursor {
        position: 0,
        data: vec![1, 2, 3, 4, 5],
    };
    assert_eq!(cursor.chunk(), &[1, 2, 3, 4, 5]);

    // Test case 2: Position is exactly at the end
    let cursor = TestCursor {
        position: 5,
        data: vec![1, 2, 3, 4, 5],
    };
    assert_eq!(cursor.chunk(), &[]);

    // Test case 3: Position exceeds the data length
    let cursor = TestCursor {
        position: 10,
        data: vec![1, 2, 3, 4, 5],
    };
    assert_eq!(cursor.chunk(), &[]);

    // Test case 4: Negative position (should never happen due to u64)
    let cursor = TestCursor {
        position: 0,
        data: vec![],
    };
    assert_eq!(cursor.chunk(), &[]);

    // Test case 5: Middle position
    let cursor = TestCursor {
        position: 2,
        data: vec![1, 2, 3, 4, 5],
    };
    assert_eq!(cursor.chunk(), &[3, 4, 5]);
}

