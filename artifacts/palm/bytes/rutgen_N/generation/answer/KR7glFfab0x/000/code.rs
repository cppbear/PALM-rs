// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Vtable {
        clone: fn() -> (),
        drop: fn() -> (),
    }

    impl fmt::Debug for Vtable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Vtable")
                .field("clone", &(self.clone as *const ()))
                .field("drop", &(self.drop as *const ()))
                .finish()
        }
    }

    let example_vtable = Vtable {
        clone: || {},
        drop: || {},
    };

    let output = format!("{:?}", example_vtable);
    assert!(output.contains("clone"));
    assert!(output.contains("drop"));
}

