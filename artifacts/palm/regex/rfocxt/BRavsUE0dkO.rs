use std::fmt;
use std::iter::repeat;
#[derive(Clone, PartialEq)]
pub enum Error {
    /// A syntax error.
    Syntax(String),
    /// The compiled program exceeded the set size limit.
    /// The argument is the size limit imposed.
    CompiledTooBig(usize),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref err) => {
                let hr: String = repeat('~').take(79).collect();
                writeln!(f, "Syntax(")?;
                writeln!(f, "{}", hr)?;
                writeln!(f, "{}", err)?;
                writeln!(f, "{}", hr)?;
                write!(f, ")")?;
                Ok(())
            }
            Error::CompiledTooBig(limit) => {
                f.debug_tuple("CompiledTooBig").field(&limit).finish()
            }
            Error::__Nonexhaustive => f.debug_tuple("__Nonexhaustive").finish(),
        }
    }
}
