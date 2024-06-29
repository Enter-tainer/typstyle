use super::PrettyPrinter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ExprScope {
    Markup,
    Statement,
    If,
    FuncCall,
}

pub(super) struct UntypedScopeGuard<'a> {
    printer: &'a PrettyPrinter,
}

impl<'a> UntypedScopeGuard<'a> {
    pub(super) fn new(printer: &'a PrettyPrinter) -> Self {
        printer.push_untyped_scope();
        Self { printer }
    }
}

impl<'a> Drop for UntypedScopeGuard<'a> {
    fn drop(&mut self) {
        self.printer.pop_scope();
    }
}

pub(super) struct ExprScopeGuard {}

impl ExprScopeGuard {
    pub(super) fn new(printer: &PrettyPrinter, scope: ExprScope) -> Self {
        printer.replace_last_untyped_scope(scope);
        Self {}
    }
}
