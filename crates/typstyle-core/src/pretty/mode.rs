use crate::PrettyPrinter;

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

impl PrettyPrinter<'_> {
    pub fn push_mode(&self, mode: Mode) {
        self.mode.borrow_mut().push(mode);
    }

    pub fn pop_mode(&self) {
        self.mode.borrow_mut().pop();
    }

    pub fn current_mode(&self) -> Mode {
        *self.mode.borrow().last().unwrap_or(&Mode::Markup)
    }
}

impl<'a> PrettyPrinter<'a> {
    pub fn with_mode(&'a self, mode: Mode) -> ModeGuard<'a> {
        self.push_mode(mode);
        ModeGuard(self)
    }

    pub fn with_mode_if(&'a self, mode: Mode, cond: bool) -> ConditionalModeGuard<'a> {
        if cond {
            self.push_mode(mode);
        }
        ConditionalModeGuard(self, cond)
    }
}

pub struct ModeGuard<'a>(&'a PrettyPrinter<'a>);

impl Drop for ModeGuard<'_> {
    fn drop(&mut self) {
        self.0.pop_mode();
    }
}

pub struct ConditionalModeGuard<'a>(&'a PrettyPrinter<'a>, bool);

impl Drop for ConditionalModeGuard<'_> {
    fn drop(&mut self) {
        if self.1 {
            self.0.pop_mode();
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    pub fn is_break_suppressed(&self) -> bool {
        self.break_suppressed.get()
    }

    pub fn suppress_breaks(&'a self) -> BreakSuppressGuard<'a> {
        BreakSuppressGuard(self, self.break_suppressed.replace(true))
    }
}

pub struct BreakSuppressGuard<'a>(&'a PrettyPrinter<'a>, bool);

impl Drop for BreakSuppressGuard<'_> {
    fn drop(&mut self) {
        self.0.break_suppressed.set(self.1);
    }
}
