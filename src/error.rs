use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Unexpected token here")]
#[diagnostic(code("Unexpected token found while lexing"), help("Try removing this"))]
pub struct UnexpectedTokenError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub err_span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Mismatched parenthesis here")]
#[diagnostic(
    code("Mismatched parenthesis found during evaluation"),
    help("Try removing this")
)]
pub struct BadParenthesesError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub err_span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Incomplete instance of a floating point number")]
#[diagnostic(
    code("Lone period found without both an integer and a fractional part"),
    help("Try completing this")
)]
pub struct IncompleteFPError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("here")]
    pub err_span: SourceSpan,
}
