pub struct CmdError {
    pub code: i32,
    pub message: &'static str,
}

impl CmdError {
    pub fn new(code: i32, message: &'static str) -> Self {
        CmdError { code, message }
    }
}

pub type CmdResult<T> = Result<T, CmdError>;

pub trait ResultExt<T> {
    fn with_cmd_err(self, code: i32, message: &'static str) -> CmdResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E> {
    fn with_cmd_err(self, code: i32, message: &'static str) -> CmdResult<T> {
        self.map_err(|_| CmdError { code, message })
    }
}

pub trait Exec {
    fn exec(self) -> CmdResult<()>;
}
