pub struct CommandError {
    pub status_code: i32,
    pub message: &'static str,
}

pub trait Exec {
    fn exec(&self) -> Result<(), CommandError>;
}
