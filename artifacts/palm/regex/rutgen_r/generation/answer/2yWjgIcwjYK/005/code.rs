// Answer 0

fn fmt_test() -> Result<(), std::fmt::Error> {
    use std::fmt;

    struct Inst {
        insts: Vec<InstEnum>,
        start: usize,
    }

    #[derive(Debug)]
    enum InstEnum {
        Bytes(BytesInst),
    }

    #[derive(Debug)]
    struct BytesInst {
        start: u8,
        end: u8,
        goto: usize,
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<InstEnum> {
            self.insts.iter()
        }
    }

    let mut output = String::new();
    let inst = Inst {
        insts: vec![
            InstEnum::Bytes(BytesInst { start: b'a', end: b'z', goto: 2 }),
            InstEnum::Bytes(BytesInst { start: b'0', end: b'9', goto: 3 }),
        ],
        start: 0,
    };

    let result = write!(output, "{}", inst);
    assert!(result.is_ok()); // Ensure that write!(f, ...) is Ok

    // Setting up a condition to check for write!(f, "\n")? returning Err or None
    let _ = write!(output, "\n").unwrap_err(); // Simulate that this line will panic

    // Returning Error for demonstration purposes since we simulate a failure in writing this
    Ok(())
}

