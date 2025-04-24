use anyhow::{anyhow, Result};

pub struct ErrorSink(Vec<String>);

impl ErrorSink {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn push(&mut self, err: impl Into<String>) {
        self.0.push(err.into());
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

#[macro_export]
macro_rules! sink_assert_eq {
    ($sink: ident, $left: expr, $right: expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $sink.push(::std::format!(
                        "assertion failed: `(left == right)`\
                        \n\
                        \n{}\
                        \n",
                        pretty_assertions::Comparison::new(left_val, right_val),
                    ))
                }
            }
        }
    };
    ($sink: ident, $left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $sink.push(::std::format!(
                        "assertion failed: `(left == right)`: {}\
                        \n\
                        \n{}\
                        \n",
                        ::std::format_args!($($arg)*),
                        pretty_assertions::Comparison::new(left_val, right_val),
                    ))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! sink_assert_str_eq {
    ($sink: ident, $left: expr, $right: expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $sink.push(::std::format!(
                        "assertion failed: `(left == right)`\
                        \n\
                        \n{}\
                        \n",
                        pretty_assertions::StrComparison::new(left_val, right_val),
                    ))
                }
            }
        }
    };
    ($sink: ident, $left: expr, $right: expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $sink.push(::std::format!(
                        "assertion failed: `(left == right)`: {}\
                        \n\
                        \n{}\
                        \n",
                        ::std::format_args!($($arg)*),
                        pretty_assertions::StrComparison::new(left_val, right_val),
                    ))
                }
            }
        }
    };
}
