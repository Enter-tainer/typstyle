use crate::PrettyPrinter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Markup,
    Code,
    Math,
}

impl PrettyPrinter {
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

pub(super) struct ModeGuard<'a>(&'a PrettyPrinter);

impl PrettyPrinter {
    pub(super) fn with_mode(&self, mode: Mode) -> ModeGuard {
        self.push_mode(mode);
        ModeGuard(self)
    }
}

impl<'a> Drop for ModeGuard<'a> {
    fn drop(&mut self) {
        self.0.pop_mode();
    }
}
