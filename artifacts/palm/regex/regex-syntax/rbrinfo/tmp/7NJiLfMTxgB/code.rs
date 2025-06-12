pub fn dot(bytes: bool) -> Hir {
        if bytes {
            let mut cls = ClassBytes::empty();
            cls.push(ClassBytesRange::new(b'\0', b'\x09'));
            cls.push(ClassBytesRange::new(b'\x0B', b'\xFF'));
            Hir::class(Class::Bytes(cls))
        } else {
            let mut cls = ClassUnicode::empty();
            cls.push(ClassUnicodeRange::new('\0', '\x09'));
            cls.push(ClassUnicodeRange::new('\x0B', '\u{10FFFF}'));
            Hir::class(Class::Unicode(cls))
        }
    }