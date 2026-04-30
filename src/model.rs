/// A simple struct to manage errors
///
/// # Fields
///
/// * `code` - The code of this error
/// * `message` - The message to show
pub struct Error {
    pub code: i32,
    pub message: &'static str,
}

impl Error {
    /// Construct new error with the given code and message
    pub fn new(code: i32, message: &'static str) -> Self {
        Error { code, message }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// An extension for std::result::Result to result management
pub trait ResultExt<T> {
    /// To map the potential std::error::Error into an crate::model::Error
    fn with_err(self, code: i32, message: &'static str) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E> {
    fn with_err(self, code: i32, message: &'static str) -> Result<T> {
        self.map_err(|_| Error { code, message })
    }
}

/// A trait for command than can be executed and can fail
pub trait Exec {
    /// Execute the command and return the eventual error
    fn exec(self, debug: bool) -> Result<()>;
}
