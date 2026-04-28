pub struct CmdError {
    pub code: i32,
    pub message: &'static str,
}

pub type CmdResult<T> = Result<T, CmdError>;

pub trait Exec {
    fn exec(self) -> CmdResult<()>;
}
