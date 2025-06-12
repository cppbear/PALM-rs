// Answer 0

#[derive(Debug)]
struct Compiler {
    compiled: Compiled,
}

#[derive(Debug)]
struct Compiled {
    is_bytes: bool,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            compiled: Compiled { is_bytes: false },
        }
    }

    pub fn bytes(mut self, yes: bool) -> Self {
        self.compiled.is_bytes = yes;
        self
    }
}

#[test]
fn test_bytes_set_to_true() {
    let compiler = Compiler::new();
    let compiled = compiler.bytes(true).compiled;
    assert_eq!(compiled.is_bytes, true);
}

#[test]
fn test_bytes_set_to_false() {
    let compiler = Compiler::new();
    let compiled = compiler.bytes(false).compiled;
    assert_eq!(compiled.is_bytes, false);
}

#[test]
fn test_bytes_initial_value() {
    let compiler = Compiler::new();
    assert_eq!(compiler.compiled.is_bytes, false);
}

