// Answer 0

#[cfg(test)]
mod tests {
    struct TranslatorBuilder {
        flags: TranslatorFlags,
    }

    struct TranslatorFlags {
        multi_line: Option<bool>,
    }

    impl TranslatorBuilder {
        fn new() -> Self {
            TranslatorBuilder {
                flags: TranslatorFlags { multi_line: None },
            }
        }

        pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    #[test]
    fn test_multi_line_enable() {
        let mut builder = TranslatorBuilder::new();
        builder.multi_line(true);
        assert_eq!(builder.flags.multi_line, Some(true));
    }

    #[test]
    fn test_multi_line_disable() {
        let mut builder = TranslatorBuilder::new();
        builder.multi_line(false);
        assert_eq!(builder.flags.multi_line, None);
    }

    #[test]
    fn test_multi_line_chain() {
        let mut builder = TranslatorBuilder::new();
        builder.multi_line(true).multi_line(false);
        assert_eq!(builder.flags.multi_line, None);
    }
}

