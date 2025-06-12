// Answer 0

#[derive(Debug)]
struct Example {
    data: Vec<u8>,
}

impl Example {
    fn inst_ptrs(&self) -> InstPtrs {
        InstPtrs {
            base: 0,
            data: &self.data[1..],
        }
    }
}

struct InstPtrs<'a> {
    base: usize,
    data: &'a [u8],
}

#[test]
fn test_inst_ptrs_non_empty_data() {
    let example = Example { data: vec![1, 2, 3, 4] };
    let inst_ptrs = example.inst_ptrs();
    assert_eq!(inst_ptrs.base, 0);
    assert_eq!(inst_ptrs.data, &[2, 3, 4]);
}

#[test]
fn test_inst_ptrs_single_element_data() {
    let example = Example { data: vec![1] };
    let result = std::panic::catch_unwind(|| example.inst_ptrs());
    assert!(result.is_err());
}

#[test]
fn test_inst_ptrs_empty_data() {
    let example = Example { data: vec![] };
    let result = std::panic::catch_unwind(|| example.inst_ptrs());
    assert!(result.is_err());
}

