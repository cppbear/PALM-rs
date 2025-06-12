// Answer 0

fn test_visit_pre_concat() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
        
        fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
    }

    struct Visitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> Visitor<'a> {
        fn new(wtr: &'a mut MockWriter) -> Self {
            Visitor { wtr }
        }

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Empty
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => {}
                _ => {}
            }
            Ok(())
        }
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn new(kind: HirKind) -> Self {
            Hir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    #[derive(Clone)]
    enum HirKind {
        Empty,
        Repetition(u32),
        Concat(Vec<Hir>),
        Alternation(Vec<Hir>),
        Literal(hir::Literal),
        Class(hir::Class),
        Anchor(hir::Anchor),
        WordBoundary(hir::WordBoundary),
        Group(hir::Group),
    }

    mod hir {
        #[derive(Clone)]
        pub enum Literal {
            Unicode(char),
            Byte(u8),
        }

        #[derive(Clone)]
        pub enum Class {
            Unicode(Vec<Range>),
            Bytes(Vec<Range>),
        }

        #[derive(Clone)]
        pub enum Anchor {
            StartLine,
            EndLine,
            StartText,
            EndText,
        }

        #[derive(Clone)]
        pub enum WordBoundary {
            Unicode,
            UnicodeNegate,
            Ascii,
            AsciiNegate,
        }

        #[derive(Clone)]
        pub enum Group {
            CaptureIndex(usize),
            CaptureName { name: String, },
            NonCapturing,
        }

        #[derive(Clone)]
        pub struct Range {
            start: char,
            end: char,
        }

        impl Range {
            pub fn start(&self) -> char {
                self.start
            }

            pub fn end(&self) -> char {
                self.end
            }
        }
    }

    let mut writer = MockWriter::new();
    let mut visitor = Visitor::new(&mut writer);

    let concat_hir = Hir::new(HirKind::Concat(vec![]));
    let result = visitor.visit_pre(&concat_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "");
    
    Ok(())
}

fn test_visit_pre_empty() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }

        fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
    }

    struct Visitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> Visitor<'a> {
        fn new(wtr: &'a mut MockWriter) -> Self {
            Visitor { wtr }
        }

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Empty
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => {}
                _ => {}
            }
            Ok(())
        }
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn new(kind: HirKind) -> Self {
            Hir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    #[derive(Clone)]
    enum HirKind {
        Empty,
        Repetition(u32),
        Concat(Vec<Hir>),
        Alternation(Vec<Hir>),
        Literal(hir::Literal),
        Class(hir::Class),
        Anchor(hir::Anchor),
        WordBoundary(hir::WordBoundary),
        Group(hir::Group),
    }

    mod hir {
        #[derive(Clone)]
        pub enum Literal {
            Unicode(char),
            Byte(u8),
        }

        #[derive(Clone)]
        pub enum Class {
            Unicode(Vec<Range>),
            Bytes(Vec<Range>),
        }

        #[derive(Clone)]
        pub enum Anchor {
            StartLine,
            EndLine,
            StartText,
            EndText,
        }

        #[derive(Clone)]
        pub enum WordBoundary {
            Unicode,
            UnicodeNegate,
            Ascii,
            AsciiNegate,
        }

        #[derive(Clone)]
        pub enum Group {
            CaptureIndex(usize),
            CaptureName { name: String, },
            NonCapturing,
        }

        #[derive(Clone)]
        pub struct Range {
            start: char,
            end: char,
        }

        impl Range {
            pub fn start(&self) -> char {
                self.start
            }

            pub fn end(&self) -> char {
                self.end
            }
        }
    }

    let mut writer = MockWriter::new();
    let mut visitor = Visitor::new(&mut writer);

    let empty_hir = Hir::new(HirKind::Empty);
    let result = visitor.visit_pre(&empty_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "");
    
    Ok(())
}

fn test_visit_pre_repetition() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }

        fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
    }

    struct Visitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> Visitor<'a> {
        fn new(wtr: &'a mut MockWriter) -> Self {
            Visitor { wtr }
        }

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Empty
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => {}
                _ => {}
            }
            Ok(())
        }
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn new(kind: HirKind) -> Self {
            Hir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    #[derive(Clone)]
    enum HirKind {
        Empty,
        Repetition(u32),
        Concat(Vec<Hir>),
        Alternation(Vec<Hir>),
        Literal(hir::Literal),
        Class(hir::Class),
        Anchor(hir::Anchor),
        WordBoundary(hir::WordBoundary),
        Group(hir::Group),
    }

    mod hir {
        #[derive(Clone)]
        pub enum Literal {
            Unicode(char),
            Byte(u8),
        }

        #[derive(Clone)]
        pub enum Class {
            Unicode(Vec<Range>),
            Bytes(Vec<Range>),
        }

        #[derive(Clone)]
        pub enum Anchor {
            StartLine,
            EndLine,
            StartText,
            EndText,
        }

        #[derive(Clone)]
        pub enum WordBoundary {
            Unicode,
            UnicodeNegate,
            Ascii,
            AsciiNegate,
        }

        #[derive(Clone)]
        pub enum Group {
            CaptureIndex(usize),
            CaptureName { name: String, },
            NonCapturing,
        }

        #[derive(Clone)]
        pub struct Range {
            start: char,
            end: char,
        }

        impl Range {
            pub fn start(&self) -> char {
                self.start
            }

            pub fn end(&self) -> char {
                self.end
            }
        }
    }

    let mut writer = MockWriter::new();
    let mut visitor = Visitor::new(&mut writer);

    let repetition_hir = Hir::new(HirKind::Repetition(3));
    let result = visitor.visit_pre(&repetition_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "");
    
    Ok(())
}

fn test_visit_pre_alternation() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_literal_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }

        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }

        fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
            self.output.push(b as char);
            Ok(())
        }
    }

    struct Visitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> Visitor<'a> {
        fn new(wtr: &'a mut MockWriter) -> Self {
            Visitor { wtr }
        }

        fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
            match *hir.kind() {
                HirKind::Empty
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => {}
                _ => {}
            }
            Ok(())
        }
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn new(kind: HirKind) -> Self {
            Hir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    #[derive(Clone)]
    enum HirKind {
        Empty,
        Repetition(u32),
        Concat(Vec<Hir>),
        Alternation(Vec<Hir>),
        Literal(hir::Literal),
        Class(hir::Class),
        Anchor(hir::Anchor),
        WordBoundary(hir::WordBoundary),
        Group(hir::Group),
    }

    mod hir {
        #[derive(Clone)]
        pub enum Literal {
            Unicode(char),
            Byte(u8),
        }

        #[derive(Clone)]
        pub enum Class {
            Unicode(Vec<Range>),
            Bytes(Vec<Range>),
        }

        #[derive(Clone)]
        pub enum Anchor {
            StartLine,
            EndLine,
            StartText,
            EndText,
        }

        #[derive(Clone)]
        pub enum WordBoundary {
            Unicode,
            UnicodeNegate,
            Ascii,
            AsciiNegate,
        }

        #[derive(Clone)]
        pub enum Group {
            CaptureIndex(usize),
            CaptureName { name: String, },
            NonCapturing,
        }

        #[derive(Clone)]
        pub struct Range {
            start: char,
            end: char,
        }

        impl Range {
            pub fn start(&self) -> char {
                self.start
            }

            pub fn end(&self) -> char {
                self.end
            }
        }
    }

    let mut writer = MockWriter::new();
    let mut visitor = Visitor::new(&mut writer);

    let alternation_hir = Hir::new(HirKind::Alternation(vec![]));
    let result = visitor.visit_pre(&alternation_hir);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "");
    
    Ok(())
}

