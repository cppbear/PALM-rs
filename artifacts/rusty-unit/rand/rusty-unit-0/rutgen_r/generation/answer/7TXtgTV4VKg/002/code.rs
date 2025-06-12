// Answer 0

#[derive(Default)]
struct TestStruct {
    index: usize,
    results: Vec<u32>,
}

impl TestStruct {
    fn generate_and_set(&mut self, count: usize) {
        for _ in 0..count {
            self.results.push(rand::random::<u32>());
        }
    }

    fn next_u64(&mut self) -> u64 {
        let read_u64 = |results: &[u32], index| {
            let data = &results[index..=index + 1];
            (u64::from(data[1]) << 32) | u64::from(data[0])
        };

        let len = self.results.as_ref().len();

        let index = self.index;
        if index < len - 1 {
            self.index += 2;
            read_u64(self.results.as_ref(), index)
        } else if index >= len {
            self.generate_and_set(2);
            read_u64(self.results.as_ref(), 0)
        } else {
            let x = u64::from(self.results.as_ref()[len - 1]);
            self.generate_and_set(1);
            let y = u64::from(self.results.as_ref()[0]);
            (y << 32) | x
        }
    }
}

#[test]
fn test_next_u64_when_index_equals_len_minus_one() {
    let mut test_struct = TestStruct {
        index: 1,
        results: vec![1, 2],
    };
    let result = test_struct.next_u64();
    assert_eq!(result, (u64::from(2) << 32) | u64::from(1));
}

#[test]
fn test_next_u64_when_index_equals_len() {
    let mut test_struct = TestStruct {
        index: 2,
        results: vec![1, 2],
    };
    let result = test_struct.next_u64();
    assert!(test_struct.results.len() >= 2);
    let read_value = |results: &[u32]| -> u64 {
        (u64::from(results[0]) << 32) | u64::from(results[1])
    };
    assert_eq!(result, read_value(&test_struct.results));
}

