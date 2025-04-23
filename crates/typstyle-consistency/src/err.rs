use anyhow::{anyhow, Result};

pub struct ErrorSink(Vec<String>);

impl ErrorSink {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn push(&mut self, err: impl Into<String>) {
        self.0.push(err.into());
    }

    pub fn check(&mut self, condition: bool, message: impl FnOnce() -> String) {
        if !condition {
            self.push(message());
        }
    }

    pub fn is_ok(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for ErrorSink {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ErrorSink> for Result<()> {
    fn from(value: ErrorSink) -> Self {
        if value.is_ok() {
            Ok(())
        } else {
            Err(anyhow!("{value}"))
        }
    }
}

impl From<&ErrorSink> for Result<()> {
    fn from(value: &ErrorSink) -> Self {
        if value.0.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("{value}"))
        }
    }
}

impl std::fmt::Display for ErrorSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} errors occurred:", self.0.len())?;
        for (i, e) in self.0.iter().enumerate() {
            let err_str = e.replace('\n', "\n    ");
            writeln!(f, "{i:4}: {err_str}")?;
        }
        Ok(())
    }
}
