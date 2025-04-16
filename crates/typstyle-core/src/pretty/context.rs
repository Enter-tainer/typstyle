#[derive(Default, Clone, Copy)]
pub struct Context {
    pub mode: Mode,
    pub break_suppressed: bool,
}

impl Context {
    pub fn with_mode(&self, mode: Mode) -> Self {
        Self { mode, ..*self }
    }

    pub fn with_mode_if(&self, mode: Mode, cond: bool) -> Self {
        Self {
            mode: if cond { mode } else { self.mode },
            ..*self
        }
    }

    pub fn suppress_breaks(&self) -> Self {
        Self {
            break_suppressed: true,
            ..*self
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Markup,
    Code,
    /// A continued code mode, where the code can freely span multiple lines.
    /// Corresponds to [`typst_syntax::parser::NewlineMode::Continue`].
    CodeCont,
    Math,
}

#[allow(unused)]
impl Mode {
    pub fn is_markup(self) -> bool {
        self == Self::Markup
    }

    /// Returns `true` if the current mode is either `Code` or `CodeCont`.
    pub fn is_code(self) -> bool {
        self == Self::Code || self == Self::CodeCont
    }

    pub fn is_code_continued(self) -> bool {
        self == Self::CodeCont
    }

    pub fn is_math(self) -> bool {
        self == Self::Math
    }
}
