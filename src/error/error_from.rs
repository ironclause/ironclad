use crate::error::IroncladError;

impl From<glob::GlobError> for IroncladError {
    fn from(value: glob::GlobError) -> Self {
        IroncladError::Glob(value)
    }
}

impl From<glob::PatternError> for IroncladError {
    fn from(value: glob::PatternError) -> Self {
        IroncladError::GlobPattern(value)
    }
}

impl From<std::io::Error> for IroncladError {
    fn from(value: std::io::Error) -> Self {
        IroncladError::StdIoError(value)
    }
}

impl From<toml::de::Error> for IroncladError {
    fn from(value: toml::de::Error) -> Self {
        IroncladError::TomlConfig(value)
    }
}