fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorKind::*;
        match *self {
            CaptureLimitExceeded => {
                write!(f, "exceeded the maximum number of \
                           capturing groups ({})", ::std::u32::MAX)
            }
            ClassEscapeInvalid => {
                write!(f, "invalid escape sequence found in character class")
            }
            ClassRangeInvalid => {
                write!(f, "invalid character class range, \
                           the start must be <= the end")
            }
            ClassRangeLiteral => {
                write!(f, "invalid range boundary, must be a literal")
            }
            ClassUnclosed => {
                write!(f, "unclosed character class")
            }
            DecimalEmpty => {
                write!(f, "decimal literal empty")
            }
            DecimalInvalid => {
                write!(f, "decimal literal invalid")
            }
            EscapeHexEmpty => {
                write!(f, "hexadecimal literal empty")
            }
            EscapeHexInvalid => {
                write!(f, "hexadecimal literal is not a Unicode scalar value")
            }
            EscapeHexInvalidDigit => {
                write!(f, "invalid hexadecimal digit")
            }
            EscapeUnexpectedEof => {
                write!(f, "incomplete escape sequence, \
                           reached end of pattern prematurely")
            }
            EscapeUnrecognized => {
                write!(f, "unrecognized escape sequence")
            }
            FlagDanglingNegation => {
                write!(f, "dangling flag negation operator")
            }
            FlagDuplicate{..} => {
                write!(f, "duplicate flag")
            }
            FlagRepeatedNegation{..} => {
                write!(f, "flag negation operator repeated")
            }
            FlagUnexpectedEof => {
                write!(f, "expected flag but got end of regex")
            }
            FlagUnrecognized => {
                write!(f, "unrecognized flag")
            }
            GroupNameDuplicate{..} => {
                write!(f, "duplicate capture group name")
            }
            GroupNameEmpty => {
                write!(f, "empty capture group name")
            }
            GroupNameInvalid => {
                write!(f, "invalid capture group character")
            }
            GroupNameUnexpectedEof => {
                write!(f, "unclosed capture group name")
            }
            GroupUnclosed => {
                write!(f, "unclosed group")
            }
            GroupUnopened => {
                write!(f, "unopened group")
            }
            NestLimitExceeded(limit) => {
                write!(f, "exceed the maximum number of \
                           nested parentheses/brackets ({})", limit)
            }
            RepetitionCountInvalid => {
                write!(f, "invalid repetition count range, \
                           the start must be <= the end")
            }
            RepetitionCountUnclosed => {
                write!(f, "unclosed counted repetition")
            }
            RepetitionMissing => {
                write!(f, "repetition operator missing expression")
            }
            UnsupportedBackreference => {
                write!(f, "backreferences are not supported")
            }
            UnsupportedLookAround => {
                write!(f, "look-around, including look-ahead and look-behind, \
                           is not supported")
            }
            _ => unreachable!(),
        }
    }