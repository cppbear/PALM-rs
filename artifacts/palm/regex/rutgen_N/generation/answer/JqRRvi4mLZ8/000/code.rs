// Answer 0

#[derive(Debug)]
struct Inst;

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn compiled_too_big(size_limit: usize) -> Self {
        Error { message: format!("Compiled size exceeds limit of {}", size_limit) }
    }
}

#[derive(Debug)]
struct Compiler {
    insts: Vec<Inst>,
    size_limit: usize,
}

impl Compiler {
    fn check_size(&self) -> Result<(), Error> {
        use std::mem::size_of;

        if self.insts.len() * size_of::<Inst>() > self.size_limit {
            Err(Error::compiled_too_big(self.size_limit))
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_check_size_within_limit() {
    let compiler = Compiler {
        insts: vec![Inst; 5], // 5 instances
        size_limit: 64,       // Assuming size_of<Inst>() is 8 bytes
    };
    let result = compiler.check_size();
    assert!(result.is_ok());
}

#[test]
fn test_check_size_exceeds_limit() {
    let compiler = Compiler {
        insts: vec![Inst; 10], // 10 instances
        size_limit: 64,       // Assuming size_of<Inst>() is 8 bytes
    };
    let result = compiler.check_size();
    assert!(result.is_err());
    if let Err(ref e) = result {
        assert_eq!(e.message, "Compiled size exceeds limit of 64");
    }
}

#[test]
fn test_check_size_exact_limit() {
    let compiler = Compiler {
        insts: vec![Inst; 8], // 8 instances; assuming size_of<Inst>() is 8 bytes
        size_limit: 64,
    };
    let result = compiler.check_size();
    assert!(result.is_ok());
}

