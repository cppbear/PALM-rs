// Answer 0

#[derive(Clone)]
struct Flags {
    // Assuming Flags has some fields; add relevant fields as necessary
}

struct Context {
    trans: Translation,
}

struct Translation {
    flags: FlagHolder,
}

struct FlagHolder {
    flags: Flags,
}

impl FlagHolder {
    fn get(&self) -> Flags {
        self.flags.clone()
    }
}

impl Context {
    fn trans(&self) -> &Translation {
        &self.trans
    }

    fn flags(&self) -> Flags {
        self.trans().flags.get()
    }
}

#[test]
fn test_flags() {
    let flags_instance = Flags {
        // Initialize fields as necessary
    };

    let flag_holder = FlagHolder {
        flags: flags_instance.clone(),
    };

    let translation = Translation {
        flags: flag_holder,
    };

    let context = Context {
        trans: translation,
    };

    let result = context.flags();
    assert_eq!(result, flags_instance);
}

