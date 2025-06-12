// Answer 0

#[test]
fn test_fmt_for_vtable() {
    use std::fmt;

    struct Vtable {
        clone: fn() -> (),
        drop: fn(),
    }

    impl fmt::Debug for Vtable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Vtable")
                .field("clone", &(self.clone as *const ()))
                .field("drop", &(self.drop as *const ()))
                .finish()
        }
    }

    let clone_fn: fn() -> () = || {};
    let drop_fn: fn() = || {};

    let vtable = Vtable {
        clone: clone_fn,
        drop: drop_fn,
    };

    let output = format!("{:?}", vtable);
    assert!(output.contains("clone"));
    assert!(output.contains("drop"));
}

