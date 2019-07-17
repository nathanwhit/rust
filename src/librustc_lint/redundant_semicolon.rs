use crate::lint::{EarlyLintPass, LintPass, EarlyContext, LintArray, LintContext};
use syntax::ast::{Stmt, StmtKind, ExprKind};

declare_lint! {
    pub REDUNDANT_SEMICOLON,
    Warn,
    "detects unnecessary trailing semicolons"
}

declare_lint_pass!(RedundantSemicolon => [REDUNDANT_SEMICOLON]);

impl EarlyLintPass for RedundantSemicolon {
    fn check_stmt(&mut self, cx: &EarlyContext<'_>, stmt: &Stmt) {
        match &stmt.node {
            StmtKind::Semi(expr) => match &expr.node {
                ExprKind::Tup(ref v) if v.is_empty() => {
                    cx.struct_span_lint(
                        REDUNDANT_SEMICOLON,
                        stmt.span,
                        "unnecessary trailing semicolon(s), consider removing"
                    ).emit();
                }
                _ => ()
            }
            _ => ()
        }
    }
}