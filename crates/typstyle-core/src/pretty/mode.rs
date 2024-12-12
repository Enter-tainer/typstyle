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
    pub(super) fn push_mode(&self, mode: Mode) {
        self.mode.borrow_mut().push(mode);
    }

    pub(super) fn pop_mode(&self) {
        self.mode.borrow_mut().pop();
    }

    pub(super) fn current_mode(&self) -> Mode {
        *self.mode.borrow().last().unwrap_or(&Mode::Markup)
    }
}

pub(super) struct ModeGuard<'a>(&'a PrettyPrinter<'a>);

impl<'a> PrettyPrinter<'a> {
    pub(super) fn with_mode(&'a self, mode: Mode) -> ModeGuard<'a> {
        self.push_mode(mode);
        ModeGuard(self)
    }
}

impl Drop for ModeGuard<'_> {
    fn drop(&mut self) {
        self.0.pop_mode();
    }
}
