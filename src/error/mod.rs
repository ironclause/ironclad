pub mod error_from;

pub enum IroncladError {
    Ok,
    /// Project errors produced when glob() scanning input files and directories
    Glob(glob::GlobError),
    /// Returned when directory scan glob pattern contained an error
    GlobPattern(glob::PatternError),
    /// Returned when file or directory read/write failed
    StdIoError(std::io::Error),
    /// Project loading error produced when loading TOML
    TomlConfig(toml::de::Error),
}

impl Default for IroncladError {
    fn default() -> Self {
        IroncladError::Ok
    }
}

impl IroncladError {
    /// Override for different error types to return different exit codes
    pub fn get_process_exit_code(&self) -> i32 {
        0
    }
}

/// Used as generic `Result<T>` which can hold any error
pub type IroncladResult<T> = Result<T, IroncladError>;

impl std::fmt::Display for IroncladError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IroncladError::Ok => write!(f, "Ok (No error)"),
            IroncladError::Glob(gerr) => gerr.fmt(f),
            IroncladError::GlobPattern(gperr) => gperr.fmt(f),
            IroncladError::StdIoError(ioerr) => writeln!(f, "{}", ioerr),
            IroncladError::TomlConfig(cfgerr) => cfgerr.fmt(f),
        }
    }
}